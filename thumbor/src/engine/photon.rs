use crate::pb::{spec::Data, *};
use bytes::Bytes;
use image::{DynamicImage, ImageBuffer, ImageOutputFormat};
use lazy_static::lazy_static;
use photon_rs::{
    effects, filters, multiple, native::open_image_from_bytes, transform, PhotonImage,
};

use super::{ImageEngine, SpecTransform};

lazy_static! {
    static ref WATERMARK: PhotonImage = {
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).expect("Couldn't open watermark image");
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

/// Photon Engine
pub struct Photon(PhotonImage);

impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&value)?))
    }
}

impl ImageEngine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        specs.iter().for_each(|s| {
            if let Some(d) = s.data.as_ref() {
                self.transform(d);
            }
        });
    }

    fn generate(self, format: image::ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

// photon 库没有提供在内存中对图片转换格式的方法，只好手工实现
fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.get_raw_pixels();
    let width = img.get_width();
    let height = img.get_height();

    let img_buffer = ImageBuffer::from_vec(width, height, raw_pixels).unwrap();
    let dynimage = DynamicImage::ImageRgba8(img_buffer);

    let mut buffer = Vec::with_capacity(32768);
    dynimage.write_to(&mut buffer, format).unwrap();
    buffer
}

impl SpecTransform<&Data> for Photon {
    fn transform(&mut self, op: &Data) {
        match op {
            spec::Data::Contrast(ref v) => self.transform(v),
            spec::Data::Crop(ref v) => self.transform(v),
            spec::Data::Filter(ref v) => self.transform(v),
            spec::Data::Fliph(ref v) => self.transform(v),
            spec::Data::Flipv(ref v) => self.transform(v),
            spec::Data::Resize(ref v) => self.transform(v),
            spec::Data::Watermark(ref v) => self.transform(v),
        }
    }
}

impl SpecTransform<&Crop> for Photon {
    fn transform(&mut self, op: &Crop) {
        let img = transform::crop(&mut self.0, op.x1, op.y1, op.x2, op.y2);
        self.0 = img;
    }
}

impl SpecTransform<&Contrast> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpecTransform<&Flipv> for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0)
    }
}

impl SpecTransform<&Fliph> for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0)
    }
}

impl SpecTransform<&Filter> for Photon {
    fn transform(&mut self, op: &Filter) {
        match filter::Filter::from_i32(op.filter) {
            Some(filter::Filter::Unspecified) => {}
            Some(f) => filters::filter(&mut self.0, f.to_str().unwrap()),
            _ => {}
        }
    }
}

impl SpecTransform<&Resize> for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::ResizeType::from_i32(op.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(
                &self.0,
                op.width,
                op.height,
                resize::SampleFilter::from_i32(op.filter).unwrap().into(),
            ),
            resize::ResizeType::SeamCarve => transform::seam_carve(&self.0, op.width, op.height),
        };
        self.0 = img;
    }
}

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }
}
