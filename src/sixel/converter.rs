use crate::errors::{AurenaError, Result};
use crate::terminal::get_terminal_size;
use crate::sixel::color::{get_palette, build_color_lookup_table};
use image::{DynamicImage};
use crossterm::style::Color;

/// convert an image to color sixel format
pub fn image_sixel_convert(img: &DynamicImage) -> Result<String> {
    let (term_w, term_h) = get_terminal_size()
        .ok_or(AurenaError::TerminalSizeError)?;

    let max_width = term_w * 6;
    let max_height = term_h * 2;
    
    let img = if img.width() > max_width || img.height() > max_height {
        img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };
    
    let palette = get_palette();
    let color_lut = build_color_lookup_table(&palette);
    let mut sixel_buffer = Vec::new();
    let mut color_bands: Vec<Vec<u8>> = vec![Vec::new(); palette.len()];
    
    video_sixel_convert(&img, &palette, &color_lut, &mut sixel_buffer, &mut color_bands)
}

/// optimized sixel conversion for video frames (reuses buffers)
pub fn video_sixel_convert(
    img: &DynamicImage, 
    palette: &[Color],
    color_lut: &[usize],
    sixel_buffer: &mut Vec<u8>,
    color_bands: &mut [Vec<u8>],
) -> Result<String> {
    const QUANT: usize = 8;
    const STEP: usize = 256 / QUANT;
    
    let (width, height) = (img.width(), img.height());
    let rgb_img = img.to_rgb8();

    sixel_buffer.clear();
    
    let estimated_size = width as usize * height as usize / 3 + palette.len() * 50;
    if sixel_buffer.capacity() < estimated_size {
        sixel_buffer.reserve(estimated_size);
    }

    sixel_buffer.extend_from_slice(b"\x1bPq");

    // define palette
    for (i, color) in palette.iter().enumerate() {
        if let Color::Rgb { r, g, b } = color {
            let r = (*r as u32 * 100) / 255;
            let g = (*g as u32 * 100) / 255;
            let b = (*b as u32 * 100) / 255;
            let palette_def = format!("#{};2;{};{};{}", i, r, g, b);
            sixel_buffer.extend_from_slice(palette_def.as_bytes());
        }
    }

    // process in 6-pixel high bands
    for y in (0..height).step_by(6) {
        // Clear color band buffers
        for band in color_bands.iter_mut() {
            band.clear();
            if band.capacity() < width as usize {
                band.reserve(width as usize);
            }
        }
        
        // build pixel data for each x position
        for x in 0..width {
            let mut color_sixels = vec![0u8; palette.len()];
            
            for bit in 0..6 {
                if y + bit < height {
                    let pixel = rgb_img.get_pixel(x, y + bit);
                    
                    // Fast color lookup using quantized LUT
                    let r_idx = (pixel[0] as usize / STEP).min(QUANT - 1);
                    let g_idx = (pixel[1] as usize / STEP).min(QUANT - 1);
                    let b_idx = (pixel[2] as usize / STEP).min(QUANT - 1);
                    let lut_idx = r_idx * QUANT * QUANT + g_idx * QUANT + b_idx;
                    
                    let color_idx = color_lut[lut_idx];
                    color_sixels[color_idx] |= 1 << bit;
                }
            }
            
            // add to appropriate color bands
            for (color_idx, &sixel_char) in color_sixels.iter().enumerate() {
                color_bands[color_idx].push(sixel_char + 63);
            }
        }
        
        // output only non-empty color bands
        for (color_idx, band) in color_bands.iter().enumerate() {
            if band.iter().any(|&c| c != 63) { // 63 is empty sixel
                let color_header = format!("#{}", color_idx);
                sixel_buffer.extend_from_slice(color_header.as_bytes());
                sixel_buffer.extend_from_slice(band);
                sixel_buffer.push(b'$');
            }
        }
        
        if y + 6 < height {
            sixel_buffer.push(b'-');
        }
    }

    sixel_buffer.extend_from_slice(b"\x1b\\");
    
    String::from_utf8(sixel_buffer.clone())
        .map_err(|e| AurenaError::SixelConversionError { 
            msg: format!("Invalid UTF-8 in sixel data: {}", e) 
        })
}