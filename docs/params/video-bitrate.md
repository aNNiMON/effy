---
icon: lucide/film
tags:
  - video
---

# Video Bitrate

Changes the video bitrate.

Expected value: number with `k` or `M` suffix.

- Default: `auto` - ffmpeg determines bitrate automatically.
- Minimal bitrate is 4k (kbps). Maximal kbps value: 9999k (kbps) for fine-tuning the bitrate up to 10M.
- Maximal bitrate is 999M (Mbps).
- `0` or `0k` resets the value to `auto`.

