mod errors;
mod terminal;
mod sixel;
mod media;

use errors::{AurenaError, Result};
use media::{image::handle_image, video::handle_video};
use clap::Parser;
use std::path::Path;

#[derive(Debug, Clone, PartialEq)]
pub enum SixelMode {
    Color,
    Monochrome,
}

impl SixelMode {
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "sixel-color" | "sixel" => Ok(SixelMode::Color),
            "sixel-mono" => Ok(SixelMode::Monochrome),
            _ => Err(AurenaError::InvalidMode {
                mode: s.to_string(),
            }),
        }
    }
}

#[derive(Parser)]
#[command(name = "aurena", version = "0.1.0", author = "Shivani Palya")]
struct CLI {
    /// input image or video file
    #[arg(long, value_name = "FILE")]
    input: String,

    /// output mode: 'sixel-color' or 'sixel-mono'
    #[arg(long, value_name = "MODE")]
    mode: String,

    /// enable audio playback for videos
    #[arg(long, action = clap::ArgAction::SetTrue)]
    audio: bool,
}

impl CLI {
    fn mode(&self) -> Result<SixelMode> {
        SixelMode::from_str(&self.mode)
    }
}

fn main() -> Result<()> {
    // initialize FFmpeg
    ffmpeg_next::init().map_err(|e| AurenaError::ProcessingError {
        msg: format!("FFmpeg initialization failed: {}", e),
    })?;

    let args = CLI::parse();
    
    // validate input file exists
    if !Path::new(&args.input).exists() {
        return Err(AurenaError::FileNotFound {
            path: args.input.clone(),
        });
    }

    let sixel_mode = args.mode()?;

    // route to appropriate handler based on file type
    if is_image_file(&args.input) {
        handle_image(&args.input, sixel_mode)?;
    } else {
        handle_video(&args.input, args.audio, sixel_mode)?;
    }

    Ok(())
}

fn is_image_file(path: &str) -> bool {
    path.ends_with(".png") || path.ends_with(".jpeg") || path.ends_with(".jpg")
}