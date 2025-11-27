---
icon: lucide/headphones
tags:
  - audio
---

# Audio Bitrate

Changes the audio bitrate.

Expected value: a bitrate number in kbps.

- Default: `auto` - ffmpeg determines bitrate automatically.
- Minimal bitrate is 4k (kbps).
- Maximal bitrate is 1024k (kbps).
- `0` resets the value to `auto`.


