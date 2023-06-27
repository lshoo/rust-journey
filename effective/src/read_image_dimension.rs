use std::path::Path;

use anyhow::Result;
use image::io::Reader;

pub fn get_image_dimension(file_path: &str) -> Result<(u32, u32)> {
    let path = Path::new(file_path);
    let reader = Reader::open(path)?;
    let dimensions = reader.into_dimensions()?;

    Ok(dimensions)
}
