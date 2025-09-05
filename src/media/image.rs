use crate::errors::{AurenaError, Result};
use crate::SixelMode;
use crate::sixel::convert_image;
use image::ImageReader;

/// handle image file processing and display
pub fn handle_image(path: &str, sixel_mode: SixelMode) -> Result<()> {
    let img = ImageReader::open(path)
        .map_err(|e| AurenaError::IoError { err: e })?
        .decode()
        .map_err(|e| AurenaError::ImageLoadError { err: e })?;
    
    let sixel_data = convert_image(&img, sixel_mode)?;
    
    println!("{}", sixel_data);
    Ok(())
}