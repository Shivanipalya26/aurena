use clap::{Arg, Command};
use ffmpeg::util::time::sleep;
use ffmpeg_next::software::scaling::{context::Context as Scaler, flag::Flags};
use ffmpeg_next::util::format::pixel::Pixel;
use ffmpeg_next::util::frame::video::Video;
use ffmpeg_next::{self as ffmpeg};
use image::{DynamicImage, ImageReader, RgbImage};
use std::{path::Path, time::Duration};
use terminal_size::{terminal_size, Height, Width};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg::init()?;

    let matches = Command::new("aurena")
        .version("0.1.0")
        .about("Image or Video to sixel converter")
        .arg(
            Arg::new("input")
                .long("input")
                .value_name("FILE")
                .help("Input image or video file")
                .required(true),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .value_name("MODE")
                .help("Output mode")
                .required(true),
        )
        .get_matches();

    let input_path = matches.get_one::<String>("input").unwrap();
    let mode = matches.get_one::<String>("mode").unwrap();

    if mode != "sixel" {
        eprintln!("Only 'sixel' mode is currently supported");
        return Ok(());
    }

    if !Path::new(input_path).exists() {
        eprintln!("File does not exist: {}", input_path);
        return Ok(());
    }

    if input_path.ends_with(".png") || input_path.ends_with(".jpeg") || input_path.ends_with(".jpg")
    {
        handle_image(input_path)?;
    } else {
        handle_video(input_path)?;
    }

    Ok(())
}

fn handle_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(path)?.decode()?;
    let sixel_data = simple_sixel_convert(&img)?;
    println!("{}", sixel_data);
    Ok(())
}

fn handle_video(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    ffmpeg::init()?;

    let mut ictx = ffmpeg::format::input(&path)?;

    let input_stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;

    let video_stream_index = input_stream.index();

    let context_decoder =
        ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Scaler::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let fps = input_stream.avg_frame_rate().0 as f64 / input_stream.avg_frame_rate().1 as f64;
    let frame_delay = Duration::from_secs_f64(1.0 / fps.max(24.0));

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            let mut frame = ffmpeg::util::frame::video::Video::empty();

            while decoder.receive_frame(&mut frame).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&frame, &mut rgb_frame)?;

                let rgb = frame_to_image(&rgb_frame);
                let img = DynamicImage::ImageRgb8(rgb);

                let sixel_data = simple_sixel_convert(&img)?;
                print!("\x1b[2J\x1b[H");
                print!("{}", sixel_data);

                let frame_delay_us = (frame_delay.as_micros() as u32).min(u32::MAX);
                sleep(frame_delay_us)?;
            }
        }
    }
    Ok(())
}

fn frame_to_image(frame: &Video) -> RgbImage {
    let mut img = RgbImage::new(frame.width(), frame.height());
    let data = frame.data(0);
    let stride = frame.stride(0) as usize;

    for (y, row) in img.enumerate_rows_mut() {
        for (x, _, pixel) in row {
            let offset = y as usize * stride + x as usize * 3;
            *pixel = image::Rgb([
                data[offset],
                data[offset + 1],
                data[offset + 2],
            ]);
        }
    }
    img
}


fn simple_sixel_convert(img: &DynamicImage) -> Result<String, Box<dyn std::error::Error>> {
    let (term_w, term_h) = get_terminal_size();

    let img = if img.width() > term_w || img.height() > term_h {
        img.resize(term_w, term_h, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };

    let (width, height) = (img.width(), img.height());
    let rgb_img = img.to_rgb8();

    let mut sixel = String::new();
    sixel.push_str("\x1bPq"); 
    sixel.push_str(&format!("\"1;1;{};{}", width, height));

    sixel.push_str("#0;2;0;0;0"); 
    sixel.push_str("#1;2;100;100;100");

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

fn get_terminal_size() -> (u32, u32) {
    if let Some((Width(cols), Height(rows))) = terminal_size() {
        let cell_w = 8;
        let cell_h = 16;

        (cols as u32 * cell_w, rows as u32 * cell_h)
    } else {
        (800, 600) 
    }
}
