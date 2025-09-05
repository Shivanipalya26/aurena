use crate::errors::{AurenaError, Result};
use crate::terminal::get_terminal_size;
use image::DynamicImage;

/// convert image to monochrome sixel format
pub fn monochrome_sixel_convert(img: &DynamicImage) -> Result<String> {
    let (term_w, term_h) = get_terminal_size()
        .ok_or(AurenaError::TerminalSizeError)?;

    let max_width = term_w * 6;
    let max_height = term_h * 3;

    let img = if img.width() > max_width || img.height() > max_height {
        img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let (width, height) = (img.width(), img.height());
    let rgb_img = img.to_rgb8();

    let mut sixel = String::new();
    sixel.push_str("\x1bPq");
    sixel.push_str(&format!("\"1;1;{};{}", width, height));

    // black and white colors
    sixel.push_str("#0;2;0;0;0");         // Black
    sixel.push_str("#1;2;100;100;100");   // White

    for y in (0..height).step_by(6) {
        sixel.push_str("#1");

        for x in 0..width {
            let mut sixel_char = 0u8;

            for bit in 0..6 {
                if y + bit < height {
                    let pixel = rgb_img.get_pixel(x, y + bit);
                    let brightness = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;

                    if brightness > 128 {
                        sixel_char |= 1 << bit;
                    }
                }
            }
            sixel.push((sixel_char + 63) as char);
        }

        sixel.push('$');
        sixel.push('-');
    }

    sixel.push_str("\x1b\\");
    Ok(sixel)
}