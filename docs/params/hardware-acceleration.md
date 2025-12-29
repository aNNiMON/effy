---
icon: lucide/gpu
tags:
  - video
---

# Hardware Acceleration

Enables hardware acceleration for processing.

- ==none==: hardware acceleration is disabled
- ==nvidia==: Cuda and NVenc hardware acceleration (windows and linux only)
- ==amd==: AMD Advanced Media Framework (AMF) hardware acceleration (windows and linux only)
- ==qsv==: Intel QSV hardware acceleration. Uses full decoding and encoding qsv acceleration if no other video filters enabled (i.e. recompress only)
- ==vaapi==: Video Acceleration API hardware acceleration (linux only)
- ==macos==: MacOS Video Toolbox hardware acceleration (macos only)
