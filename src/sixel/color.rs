use crossterm::style::Color;
use image::Rgb;

/// generate color palette for sixel rendering
pub fn get_palette() -> Vec<Color> {
    vec![
        // Grayscale gradient (15 colors)
        Color::Rgb { r: 0, g: 0, b: 0 },       // Black
        Color::Rgb { r: 16, g: 16, b: 16 },
        Color::Rgb { r: 32, g: 32, b: 32 },
        Color::Rgb { r: 48, g: 48, b: 48 },
        Color::Rgb { r: 64, g: 64, b: 64 },
        Color::Rgb { r: 80, g: 80, b: 80 },
        Color::Rgb { r: 96, g: 96, b: 96 },
        Color::Rgb { r: 128, g: 128, b: 128 },
        Color::Rgb { r: 160, g: 160, b: 160 },
        Color::Rgb { r: 176, g: 176, b: 176 },
        Color::Rgb { r: 192, g: 192, b: 192 },
        Color::Rgb { r: 208, g: 208, b: 208 },
        Color::Rgb { r: 224, g: 224, b: 224 },
        Color::Rgb { r: 240, g: 240, b: 240 },
        Color::Rgb { r: 255, g: 255, b: 255 }, // White
        
        // Red spectrum (7 colors)
        Color::Rgb { r: 64, g: 0, b: 0 },
        Color::Rgb { r: 128, g: 0, b: 0 },
        Color::Rgb { r: 192, g: 0, b: 0 },
        Color::Rgb { r: 255, g: 0, b: 0 },
        Color::Rgb { r: 255, g: 64, b: 64 },
        Color::Rgb { r: 255, g: 128, b: 128 },
        Color::Rgb { r: 255, g: 192, b: 192 },
        
        // Green spectrum (7 colors)
        Color::Rgb { r: 0, g: 64, b: 0 },
        Color::Rgb { r: 0, g: 128, b: 0 },
        Color::Rgb { r: 0, g: 192, b: 0 },
        Color::Rgb { r: 0, g: 255, b: 0 },
        Color::Rgb { r: 64, g: 255, b: 64 },
        Color::Rgb { r: 128, g: 255, b: 128 },
        Color::Rgb { r: 192, g: 255, b: 192 },
        
        // Blue spectrum (7 colors)
        Color::Rgb { r: 0, g: 0, b: 64 },
        Color::Rgb { r: 0, g: 0, b: 128 },
        Color::Rgb { r: 0, g: 0, b: 192 },
        Color::Rgb { r: 0, g: 0, b: 255 },
        Color::Rgb { r: 64, g: 64, b: 255 },
        Color::Rgb { r: 128, g: 128, b: 255 },
        Color::Rgb { r: 192, g: 192, b: 255 },
        
        // Yellow spectrum (4 colors)
        Color::Rgb { r: 128, g: 128, b: 0 },
        Color::Rgb { r: 192, g: 192, b: 0 },
        Color::Rgb { r: 255, g: 255, b: 0 },
        Color::Rgb { r: 255, g: 255, b: 128 },
        
        // Magenta spectrum (4 colors)
        Color::Rgb { r: 128, g: 0, b: 128 },
        Color::Rgb { r: 192, g: 0, b: 192 },
        Color::Rgb { r: 255, g: 0, b: 255 },
        Color::Rgb { r: 255, g: 128, b: 255 },
        
        // Cyan spectrum (4 colors)
        Color::Rgb { r: 0, g: 128, b: 128 },
        Color::Rgb { r: 0, g: 192, b: 192 },
        Color::Rgb { r: 0, g: 255, b: 255 },
        Color::Rgb { r: 128, g: 255, b: 255 },
        
        // Orange spectrum (4 colors)
        Color::Rgb { r: 128, g: 64, b: 0 },
        Color::Rgb { r: 192, g: 96, b: 0 },
        Color::Rgb { r: 255, g: 128, b: 0 },
        Color::Rgb { r: 255, g: 192, b: 128 },
        
        // Purple spectrum (4 colors)
        Color::Rgb { r: 64, g: 0, b: 128 },
        Color::Rgb { r: 128, g: 0, b: 192 },
        Color::Rgb { r: 160, g: 32, b: 240 },
        Color::Rgb { r: 192, g: 128, b: 255 },
        
        // Mixed colors (8 colors)
        Color::Rgb { r: 128, g: 255, b: 192 }, // Mint
        Color::Rgb { r: 255, g: 192, b: 128 }, // Peach
        Color::Rgb { r: 192, g: 255, b: 128 }, // Light Green-Yellow
        Color::Rgb { r: 128, g: 192, b: 255 }, // Sky Blue
        Color::Rgb { r: 255, g: 128, b: 192 }, // Pink
        Color::Rgb { r: 192, g: 128, b: 192 }, // Mauve
        Color::Rgb { r: 128, g: 192, b: 128 }, // Sage Green
        Color::Rgb { r: 192, g: 192, b: 128 }, // Tan

        // Skin tones and flesh colors (8 colors)
        Color::Rgb { r: 255, g: 220, b: 177 }, // Light skin
        Color::Rgb { r: 241, g: 194, b: 125 }, // Medium light skin
        Color::Rgb { r: 224, g: 172, b: 105 }, // Medium skin
        Color::Rgb { r: 198, g: 134, b: 66 },  // Medium dark skin
        Color::Rgb { r: 141, g: 85, b: 36 },   // Dark skin
        Color::Rgb { r: 92, g: 51, b: 23 },    // Very dark skin
        Color::Rgb { r: 255, g: 192, b: 203 }, // Pink
        Color::Rgb { r: 255, g: 160, b: 122 }, // Light salmon
    ]
}

/// build a color lookup table for fast color quantization
pub fn build_color_lookup_table(palette: &[Color]) -> Vec<usize> {
    const QUANT: usize = 8; // 8x8x8 = 512 entries
    const STEP: usize = 256 / QUANT;
    
    let mut lut = vec![0; QUANT * QUANT * QUANT];
    
    for r in 0..QUANT {
        for g in 0..QUANT {
            for b in 0..QUANT {
                let rgb = [
                    (r * STEP).min(255) as u8,
                    (g * STEP).min(255) as u8,
                    (b * STEP).min(255) as u8,
                ];
                let pixel = Rgb(rgb);
                let color_idx = nearest_color_weighted(&pixel, palette);
                let lut_idx = r * QUANT * QUANT + g * QUANT + b;
                lut[lut_idx] = color_idx;
            }
        }
    }
    lut
}

/// find the nearest color using perceptual weighting
pub fn nearest_color_weighted(pixel: &Rgb<u8>, palette: &[Color]) -> usize {
    let mut min_dist = f64::MAX;
    let mut idx = 0;

    for (i, color) in palette.iter().enumerate() {
        if let Color::Rgb { r, g, b } = color {
            let dr = pixel[0] as f64 - *r as f64;
            let dg = pixel[1] as f64 - *g as f64;
            let db = pixel[2] as f64 - *b as f64;
            
            // Enhanced perceptual weighting (based on human eye sensitivity)
            let dist = 0.21 * dr * dr + 0.72 * dg * dg + 0.07 * db * db;

            if dist < min_dist {
                min_dist = dist;
                idx = i;
            }
        }
    }

    idx
}