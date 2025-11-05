# effy

TUI FFmpeg helper

## Features

- change a video resolution, bitrate, frame rate, speed
- change an audio bitrate, volume, pitch, tempo
- apply audio effects (crystalizer)
- remove audio from the video
- trim video/audio

## Usage

> [!IMPORTANT]
> `ffmpeg` and `ffprobe` must be [installed](https://ffmpeg.org/download.html) and available in the `PATH` env variable

```bash
effy input.mp4
effy "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
```

## Install

Download pre-compiled binaries from [Releases](https://github.com/aNNiMON/effy/releases)

Or compile from source:

1. Install [Rustup](https://rustup.rs/)
2. Clone this repo
3. `cargo build --release`
4. Navigate to `./target/release` and look for `effy` binary


## Telegram bot

If you're looking for a self-hosted server-side alternative that works with Telegram media, look no further: [effybot](https://github.com/aNNiMON/effybot)
