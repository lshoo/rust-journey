use anyhow::Result;
use effective::download_file::download_file;

#[tokio::main]
async fn main() -> Result<()> {
    let image_url = "https://www.rust-lang.org/static/images/rust-logo-blk.svg";
    let file_name = "rust-logo-blk.svg";
    match download_file(image_url, file_name).await {
        Ok(_) => println!("download image sucessfully"),
        Err(e) => println!("download image failed: {:?}", e),
    };

    Ok(())
}
