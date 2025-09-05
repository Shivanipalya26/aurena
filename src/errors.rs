use std::{error::Error, fmt::Display, io};

#[derive(Debug)]
pub enum AurenaError {
    FileNotFound { path: String },
    IoError { err: io::Error },
    ImageLoadError { err: image::ImageError },
    VideoOpenError { path: String },
    VideoStreamError { err: ffmpeg_next::Error },
    VideoDecodingError { msg: String },
    AudioExtractionError { err: Box<dyn Error> },
    AudioPlaybackError { err: rodio::PlayError },
    AudioStreamError { err: rodio::StreamError },
    AudioDecoderError { err: rodio::decoder::DecoderError },
    SixelConversionError { msg: String },
    TerminalSizeError,
    InvalidMode { mode: String },
    ProcessingError { msg: String },
}

impl Display for AurenaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AurenaError::FileNotFound { path } => {
                writeln!(f, "File not found: {}", path)
            }
            AurenaError::IoError { err } => {
                writeln!(f, "I/O error: {}", err)
            }
            AurenaError::ImageLoadError { err } => {
                writeln!(f, "Failed to load image: {}", err)
            }
            AurenaError::VideoOpenError { path } => {
                writeln!(f, "Failed to open video: {}", path)
            }
            AurenaError::VideoStreamError { err } => {
                writeln!(f, "Video stream error: {}", err)
            }
            AurenaError::VideoDecodingError { msg } => {
                writeln!(f, "Video decoding error: {}", msg)
            }
            AurenaError::AudioExtractionError { err } => {
                writeln!(f, "Audio extraction failed: {}", err)
            }
            AurenaError::AudioPlaybackError { err } => {
                writeln!(f, "Audio playback error: {}", err)
            }
            AurenaError::AudioStreamError { err } => {
                writeln!(f, "Audio stream error: {}", err)
            }
            AurenaError::AudioDecoderError { err } => {
                writeln!(f, "Audio decoder error: {}", err)
            }
            AurenaError::SixelConversionError { msg } => {
                writeln!(f, "Sixel conversion error: {}", msg)
            }
            AurenaError::TerminalSizeError => {
                writeln!(f, "Failed to detect terminal size")
            }
            AurenaError::InvalidMode { mode } => {
                writeln!(f, "Invalid mode: {}", mode)
            }
            AurenaError::ProcessingError { msg } => {
                writeln!(f, "Processing error: {}", msg)
            }
        }
    }
}

impl Error for AurenaError {}

pub type Result<T> = std::result::Result<T, AurenaError>;