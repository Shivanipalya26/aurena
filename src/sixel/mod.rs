pub mod color;
pub mod converter;
pub mod monochrome;

pub use converter::image_sixel_convert;
pub use monochrome::monochrome_sixel_convert;

use crate::SixelMode;
use crate::errors::Result;
use image::DynamicImage;

pub fn convert_image(img: &DynamicImage, mode: SixelMode) -> Result<String> {
    match mode {
        SixelMode::Color => image_sixel_convert(img),
        SixelMode::Monochrome => monochrome_sixel_convert(img),
    }
}
