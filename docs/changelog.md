---
icon: lucide/scroll-text
---

# Changelog

See also [GitHub Releases](https://github.com/aNNiMON/effy/releases)


## 0.6.0

- Added VAAPI and AMF hardware acceleration options
- Ability to stop running ffmpeg process
- Portrait mode support
- Colorize Info pane items based on media streams
- Hints improvements


## 0.5.0 - Custom values, Output parameter

* Ability to set custom value for parameters by pressing Enter
* Added Output parameter ([docs](https://projects.annimon.com/projects/effy/params/output-format/)). It allows to specify a different output format, for example, extract audio from the given video source.
* Adjust precise trim time based on the speed factor
* Better extension defaults for URL inputs


## v0.4.0 - Trim parameter

- Added a Trim parameter that allows to trim media by duration or start/end time
- Added more common Video frame rate values
- Shift+Tab for reverse focus switch


## v0.3.0 - Hardware acceleration, URL input

- Added hardware acceleration parameter. Supports nvidia and intel qsv
- URL input support:
  ```bash
  effy "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
  ```
- 'Render as' dialog redesign with hints


## v0.2.0 - Core rework

* Core rework with different parameters model
* Added 'Save as' modal dialog
* Redraw on resize
