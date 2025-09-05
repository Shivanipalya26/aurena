# Aurena

A high-performance, minimal terminal-based tool that converts images, GIFs, and videos to sixel graphics with optional audio playback support.

## Features

- 🖼️ **Image Display**: Convert PNG, JPEG, and JPG images to sixel format
- 🎬 **Video Playback**: Play videos in your terminal with frame-rate synchronization
- 🎵 **Audio Support**: Synchronized audio playback for videos 
- ⚡ **Performance**: Efficient color quantization and lookup tables
- 🎨 **Dual Rendering Modes**: 
  - Color mode with 64-color optimized palette
  - Monochrome mode for better performance


## Installation

### Prerequisites

- Rust (install via [rustup](https://rustup.rs/))
- FFmpeg (for video processing and audio extraction)
- A sixel-compatible terminal (see [Compatible Terminals](#compatible-terminals))

### Installing FFmpeg and Dependencies

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install ffmpeg libsixel-dev libasound2-dev libpulse-dev libssl-dev
```

**Arch Linux:**
```bash
sudo pacman -S ffmpeg libsixel alsa-lib libpulse
```

**macOS:**
```bash
brew install ffmpeg libsixel
```

### Building from Source

```bash
git clone https://github.com/Shivanipalya26/aurena.git
cd aurena
cargo build --release
```

The binary will be available at `target/release/aurena`.

## Usage

### Basic Commands

```bash
# Display an image in color mode
cargo run -- --input image/image.jpg --mode sixel-color

# Display an image in monochrome mode
cargo run -- --input image/image.png --mode sixel-mono

# Play video with audio
cargo run -- --input video/video.mp4 --mode sixel-color --audio

# Play video without audio
cargo run -- --input video/video.mp4 --mode sixel-color

# Play video with audio in monochrome mode
cargo run -- --input video/video.mp4 --mode sixel-mono --audio
```

### Command Line Options

| Option | Description | Required | Default |
|--------|-------------|----------|---------|
| `--input FILE` | Input image or video file path | Yes | - |
| `--mode MODE` | Rendering mode: `sixel-color` or `sixel-mono` | Yes | - |
| `--audio` | Enable audio playback for videos | No | Disabled |

### Supported Formats

**Images:**
- PNG
- JPEG/JPG

**Videos:**
- Any format supported by FFmpeg (MP4, AVI, MKV, WebM, etc.)

**Audio:**
- Automatic extraction from video files
- External audio files: WAV, MP3, FLAC, OGG, AAC, M4A

## Compatible Terminals

Aurena requires a terminal with sixel graphics support:

| Terminal | SIXEL Support | Command |
|----------|---------------|---------|
| **xterm** | Native | `xterm -ti vt340` |
| **mlterm** | Native | Default |
| **wezterm** | Configurable | Enable in config |
| **foot** | Native | Default |
| **mintty** | Optional | `--enable-sixel` |
| **iTerm2** | Beta | Enable in preferences |

### Testing Terminal Compatibility

```bash
# Test if your terminal supports sixel
echo -e '\ePq#0;2;0;0;0#1;2;100;100;100#1~~@@vv@@~~@@~~$#0??}}GG}}??}}??-\e\\'
```

You should see a small test pattern if sixel is supported.

## Performance & Quality

### Rendering Modes

**Color Mode (`sixel-color`):**
- 64-color optimized palette
- Perceptual color weighting for better image quality
- Skin tone preservation
- Best for photographs and complex images
- Audio playback supported

**Monochrome Mode (`sixel-mono`):**
- Black and white dithering
- Higher performance
- Better for text documents or simple graphics
- No audio support (by design)

### Optimization Features

- **Color Quantization**: 8×8×8 lookup table for fast color mapping
- **Frame Rate Control**: Maintains original video 
- **Memory Efficiency**: Reusable buffers and minimal allocations

## Troubleshooting

### Common Issues

#### Failed to open video
```bash
# Check file exists and permissions
ls -la /aurena/video/video.mp4

# Verify codec support
ffmpeg -codecs | grep h264

# Test playback with ffplay 
ffplay /aurena/video/video.mp4

# Check decoding support without playing
ffmpeg -i /aurena/video/video.mp4 -f null -
```

#### Audio/video sync issues
```bash
# Check ffplay installation
which ffplay

# Test audio playback separately
ffplay -nodisp -autoexit /aurena/video/video.mp4
```

## Technical Details

### Color Palette
The color mode uses a carefully crafted 64-color palette including:
- 15-step grayscale gradient
- Primary color spectrums (RGB)
- Secondary colors (CMY)
- Mixed tones and skin colors
- Perceptually weighted color matching

### Architecture
- **FFmpeg Integration**: Video decoding and scaling
- **Sixel Generation**: Custom optimized encoder
- **Audio Pipeline**: Rodio-based playback with synchronization

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Video processing powered by [FFmpeg](https://ffmpeg.org/)
- Audio playback via [Rodio](https://github.com/RustAudio/rodio)
- Image processing with [image](https://github.com/image-rs/image)
- Terminal interaction through [crossterm](https://github.com/crossterm-rs/crossterm)

---

**Version**: 0.1.0  
**Author**: [Shivani Palya]  