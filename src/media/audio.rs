use crate::errors::{AurenaError, Result};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{
    fs::File,
    io::BufReader,
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

/// setup audio playback for a video file
pub fn audio_setup(video_path: &str) -> Result<(Arc<Sink>, OutputStream)> {
    println!("Setting up audio for: {}", video_path);
    
    let audio_path = find_or_extract_audio(video_path)?;
    
    let (stream, stream_handle) = OutputStream::try_default()
        .map_err(|e| AurenaError::AudioStreamError { err: e })?;
    
    let sink = Arc::new(Sink::try_new(&stream_handle)
        .map_err(|e| AurenaError::AudioPlaybackError { err: e })?);
    
    let file = File::open(&audio_path)
        .map_err(|e| AurenaError::IoError { err: e })?;
    
    let source = Decoder::new(BufReader::new(file))
        .map_err(|e| AurenaError::AudioDecoderError { err: e })?;
    
    println!("Audio: {} channels, {} Hz", source.channels(), source.sample_rate());
    
    sink.append(source);
    sink.pause();
    
    // cleanup handling for temporary files
    if audio_path != video_path && audio_path.contains("temp_audio") {
        let audio_path_clone = audio_path.clone();
        let sink_clone = Arc::clone(&sink);
        thread::spawn(move || {
            let cleanup_start = Instant::now();
            let max_wait = Duration::from_secs(300);
            
            loop {
                if sink_clone.empty() || cleanup_start.elapsed() > max_wait {
                    break;
                }
                thread::sleep(Duration::from_secs(1));
            }
            
            thread::sleep(Duration::from_secs(5));
            
            if let Err(e) = std::fs::remove_file(&audio_path_clone) {
                eprintln!("Cleanup failed: {}", e);
            } else {
                println!("Temp audio cleaned up");
            }
        });
    }
    
    Ok((sink, stream))
}

/// find existing audio file or extract from video
fn find_or_extract_audio(video_path: &str) -> Result<String> {
    let video_stem = std::path::Path::new(video_path).file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("video");
    
    let video_dir = std::path::Path::new(video_path).parent()
        .unwrap_or(std::path::Path::new("."));
    
    let audio_extensions = ["wav", "mp3", "flac", "ogg", "aac", "m4a"];
    
    // look for existing audio files
    for ext in &audio_extensions {
        let audio_path = video_dir.join(format!("{}.{}", video_stem, ext));
        if audio_path.exists() {
            println!("Found separate audio: {:?}", audio_path);
            return Ok(audio_path.to_string_lossy().into_owned());
        }
    }
    
    // extract audio from video
    println!("Extracting audio from video");
    let temp_audio_path = format!("temp_audio_{}.wav", std::process::id());
    
    extract_audio(video_path, &temp_audio_path)?;
    
    Ok(temp_audio_path)
}

/// extract audio from video file using FFmpeg
fn extract_audio(video_path: &str, output_path: &str) -> Result<()> {
    use std::process::Command;
    
    let output = Command::new("ffmpeg")
        .args(&[
            "-i", video_path,
            "-vn",
            "-acodec", "pcm_s16le", 
            "-ar", "44100",
            "-f", "wav",
            "-y",
            output_path
        ])
        .output()
        .map_err(|e| AurenaError::AudioExtractionError { 
            err: Box::new(e) 
        })?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(AurenaError::AudioExtractionError { 
            err: Box::new(std::io::Error::new(
                std::io::ErrorKind::Other, 
                format!("Audio extraction failed: {}", error)
            )) 
        });
    }
    
    let metadata = std::fs::metadata(output_path)
        .map_err(|e| AurenaError::IoError { err: e })?;
    
    if metadata.len() == 0 {
        return Err(AurenaError::AudioExtractionError { 
            err: Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData, 
                "Empty audio file generated"
            )) 
        });
    }
    
    Ok(())
}