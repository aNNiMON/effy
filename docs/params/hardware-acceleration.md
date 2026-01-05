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


!!! info "Important (Linux)"

    The VAAPI device `/dev/dri/renderD128` is used by default. To override this, set the `EFFY_VAAPI_DEVICE` environment variable.  
    Refer to [FFmpeg VAAPI page](https://trac.ffmpeg.org/wiki/Hardware/VAAPI) for more details.

