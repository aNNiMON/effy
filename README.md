# effy

A small and friendly terminal FFmpeg helper that simplifies common tasks.

[![Documentation](https://img.shields.io/badge/documentation-blue?style=for-the-badge)](https://projects.annimon.com/projects/effy)
[![GitHub Release](https://img.shields.io/github/v/release/annimon/effy?include_prereleases&sort=semver&style=for-the-badge)](https://github.com/aNNiMON/effy/releases)
[![](https://img.shields.io/badge/Ratatui-000?logo=ratatui&logoColor=fff&style=for-the-badge)](https://ratatui.rs/)

![effy TUI](../docs/docs/assets/effy.png?raw=true)

## Features

- change a video resolution, bitrate, frame rate, speed
- change an audio bitrate, volume, pitch, tempo
- apply audio effects (crystalizer)
- extract or remove audio from the video
- trim video/audio
- use hardware acceleration
- apply presets (preload in the UI, or immediately from the CLI)

## Usage

> [!IMPORTANT]
> `ffmpeg` and `ffprobe` must be [installed](https://ffmpeg.org/download.html) and available in the `PATH` env variable

```bash
effy input.mp4
effy "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"

# Presets
effy --preset "noaudio:1;scale:250;output:mp4" input.mp4
effy --preset "noaudio:1;scale:250;output:mp4" --apply input.mp4
```

## Install

Download pre-compiled binaries from [Releases](https://github.com/aNNiMON/effy/releases)

Or install using `cargo`:

```bash
cargo install --locked effy
```

Or compile from source:

1. Install [Rustup](https://rustup.rs/)
2. Clone this repo
3. `cargo build --release`
4. Navigate to `./target/release` and look for `effy` binary


## Telegram bot

If you're looking for a self-hosted server-side alternative that works with Telegram media, look no further: [effybot](https://github.com/aNNiMON/effybot)
