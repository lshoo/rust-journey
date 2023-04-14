use anyhow::Result;
use axum::{
    extract::Path,
    http::{HeaderMap, HeaderValue},
    routing::get,
    Extension, Router,
};
use bytes::Bytes;
use image::ImageOutputFormat;
use lru::LruCache;
use percent_encoding::{percent_decode_str, percent_encode, NON_ALPHANUMERIC};
use reqwest::StatusCode;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    num::NonZeroUsize,
    sync::Arc,
};
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tracing::{info, instrument};

use thumbor::{
    engine::{ImageEngine, Photon},
    pb::{filter, resize, ImageSpec, Spec},
};

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cache: Cache = Arc::new(Mutex::new(LruCache::new(NonZeroUsize::new(1024).unwrap())));

    let router = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(ServiceBuilder::new().layer(Extension(cache)).into_inner());
    let addr = "0.0.0.0:3000".parse().unwrap();

    tracing::debug!("listening on {}", addr);

    print_test_url("https://images.pexels.com/photos/1562477/pexels-photo-1562477.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260");
    print_test_url("https://images.pexels.com/photos/1200229/pexels-photo-1200229.jpeg?auto=compress&cs=tinysrgb&w=1260&h=750");

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Server failed");
}

async fn generate(
    Path((spec, url)): Path<(String, String)>,
    Extension(cache): Extension<Cache>,
) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    info!("Got url: {url}");

    let url = percent_decode_str(&url).decode_utf8_lossy();

    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let data = retrieve_image(&url, cache)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut engine: Photon = data
        .try_into()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    engine.apply(&spec.specs);

    let image = engine.generate(ImageOutputFormat::Jpeg(85));

    let mut headers = HeaderMap::new();
    headers.insert("content-type", HeaderValue::from_static("image/jpeg"));

    Ok((headers, image))
}

// `info` will only take effect when RUST_LOG=INFO， and skip the args `url` and `cache` value
#[instrument(level = "info", skip(url, cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);

    let key = hasher.finish();

    let g = &mut cache.lock().await;
    let data = match g.get(&key) {
        Some(v) => {
            info!("Match cache {key}");
            v.to_owned()
        }
        None => {
            info!("Retrieve url");
            let rsp = reqwest::get(url).await?;
            let data = rsp.bytes().await?;
            g.put(key, data.clone());
            info!("cache updated");
            data
        }
    };

    Ok(data)
}

fn print_test_url(url: &str) {
    use std::borrow::Borrow;
    let spec1 = Spec::new_resize(500, 800, resize::SampleFilter::CatmullRom);
    let spec2 = Spec::new_watermark(20, 20);
    let spec3 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2, spec3]);
    let s: String = image_spec.borrow().into();
    let test_image = percent_encode(url.as_bytes(), NON_ALPHANUMERIC).to_string();
    println!("test url: http://localhost:3000/image/{}/{}", s, test_image);
}
