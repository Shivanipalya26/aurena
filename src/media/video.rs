use crate::errors::{AurenaError, Result};
use crate::SixelMode;
use crate::terminal::{get_terminal_size, clear_screen, flush_display};
use crate::sixel::color::{get_palette, build_color_lookup_table};
use crate::sixel::{converter::video_sixel_convert, monochrome::monochrome_sixel_convert};
use crate::media::audio::audio_setup;
use image::{DynamicImage, RgbImage};
use ffmpeg_next::{
    self as ffmpeg,
    software::scaling::{context::Context as Scaler, flag::Flags},
    util::{
        format::pixel::Pixel,
        frame::video::Video,
    },
};
use std::time::{Duration, Instant};

/// handle video file processing and playback
pub fn handle_video(path: &str, enable_audio: bool, sixel_mode: SixelMode) -> Result<()> {
    ffmpeg::init()
        .map_err(|e| AurenaError::VideoStreamError { err: e })?;

    let mut ictx = ffmpeg::format::input(&path)
        .map_err(|_e| AurenaError::VideoOpenError { 
            path: path.to_string() 
        })?;
    
    let input_stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(AurenaError::VideoDecodingError { 
            msg: "No video stream found".to_string() 
        })?;

    let video_stream_index = input_stream.index();
    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())
        .map_err(|e| AurenaError::VideoStreamError { err: e })?;
    let mut decoder = context_decoder.decoder().video()
        .map_err(|e| AurenaError::VideoStreamError { err: e })?;

    let (term_w, term_h) = get_terminal_size()
        .ok_or(AurenaError::TerminalSizeError)?;
    
    let scale_factor = f64::min(
        term_w as f64 / decoder.width() as f64,
        term_h as f64 / decoder.height() as f64
    ).min(1.0);
    
    let target_width = (decoder.width() as f64 * scale_factor) as u32;
    let target_height = (decoder.height() as f64 * scale_factor) as u32;

    let mut scaler = Scaler::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        target_width,
        target_height,
        Flags::FAST_BILINEAR,
    ).map_err(|e| AurenaError::VideoStreamError { err: e })?;

    let fps = input_stream.avg_frame_rate().0 as f64 / input_stream.avg_frame_rate().1 as f64;
    let frame_duration = Duration::from_secs_f64(1.0 / fps.max(30.0));

    // audio setup only for color mode 
    let (audio_sink, _audio_stream) = if enable_audio {
        match audio_setup(path) {
            Ok((sink, stream)) => (Some(sink), Some(stream)),
            Err(e) => {
                eprintln!("Audio setup failed: {}. Continuing without audio.", e);
                (None, None)
            }
        }
    } else {
        println!("Audio disabled");
        (None, None)
    };

    // setup conversion buffers based on mode
    let (palette, color_lut, mut sixel_buffer, mut color_bands) = match sixel_mode {
        SixelMode::Color => {
            let palette = get_palette();
            let color_lut = build_color_lookup_table(&palette);
            let sixel_buffer = Vec::with_capacity((target_width * target_height / 2) as usize);
            let color_bands: Vec<Vec<u8>> = vec![Vec::with_capacity(target_width as usize); palette.len()];
            (Some(palette), Some(color_lut), sixel_buffer, Some(color_bands))
        },
        SixelMode::Monochrome => {
            (None, None, Vec::new(), None)
        }
    };

    let mut frame_count = 0;
    let start_time = Instant::now();
    
    // start audio playback
    if let Some(ref sink) = audio_sink {
        sink.play();
    }

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)
                .map_err(|e| AurenaError::VideoDecodingError { 
                    msg: format!("Failed to send packet: {}", e) 
                })?;
            
            let mut frame = Video::empty();

            while decoder.receive_frame(&mut frame).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&frame, &mut rgb_frame)
                    .map_err(|e| AurenaError::VideoStreamError { err: e })?;

                let rgb = frame_to_image(&rgb_frame);
                let img = DynamicImage::ImageRgb8(rgb);

                let sixel_data = match sixel_mode {
                    SixelMode::Color => {
                        video_sixel_convert(
                            &img, 
                            palette.as_ref().unwrap(),
                            color_lut.as_ref().unwrap(),
                            &mut sixel_buffer,
                            color_bands.as_mut().unwrap()
                        )?
                    },
                    SixelMode::Monochrome => {
                        monochrome_sixel_convert(&img)?
                    }
                };
                
                clear_screen();
                print!("{}", sixel_data);
                flush_display().map_err(|e| AurenaError::IoError { err: e })?;

                frame_count += 1;
                
                let target_time = start_time + frame_duration * frame_count;
                let current_time = Instant::now();
                
                if target_time > current_time {
                    let sleep_duration = target_time - current_time;
                    if sleep_duration < Duration::from_millis(100) {
                        std::thread::sleep(sleep_duration);
                    }
                }
                
                if frame_count % (fps as u32 * 5) == 0 {
                    let elapsed = start_time.elapsed();
                    let actual_fps = frame_count as f64 / elapsed.as_secs_f64();
                    let video_time = elapsed.as_secs();
                    
                    if let Some(ref sink) = audio_sink {
                        let is_paused = sink.is_paused();
                        println!("\r{}:{:02} | Frame: {} | FPS: {:.1} | Audio: {}", 
                                video_time / 60, video_time % 60,
                                frame_count, actual_fps,
                                if is_paused { "PAUSED" } else { "PLAYING" });
                    } else {
                        println!("\r{}:{:02} | Frame: {} | FPS: {:.1}", 
                                video_time / 60, video_time % 60,
                                frame_count, actual_fps);
                    }
                }
            }
        }
    }

    // audio cleanup
    if let Some(sink) = audio_sink {
        let audio_wait_start = Instant::now();
        let max_wait = Duration::from_secs(5);
        
        while !sink.empty() && audio_wait_start.elapsed() < max_wait {
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    
    Ok(())
}

/// convert FFmpeg video frame to RgbImage
fn frame_to_image(frame: &Video) -> RgbImage {
    let mut img = RgbImage::new(frame.width(), frame.height());
    let data = frame.data(0);
    let stride = frame.stride(0) as usize;

    for (y, row) in img.enumerate_rows_mut() {
        for (x, _, pixel) in row {
            let offset = y as usize * stride + x as usize * 3;
            *pixel = image::Rgb([data[offset], data[offset + 1], data[offset + 2]]);
        }
    }
    img
}