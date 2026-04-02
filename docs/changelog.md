---
icon: lucide/scroll-text
---

# Changelog

See also [:simple-github: GitHub Releases](https://github.com/aNNiMON/effy/releases)

## 0.9.0 - Trim improvement, Copy modal

- Added percent values support to Trim parameter. Works only if the media duration is known
- Negative `Start` and `To` values are considered relative to the duration
- Show duration in trim modal if present
- Trim parameter support in presets
- :lucide-circle-alert:{ .ic-warn } Preset value separator changed from `:` to `=`
- Added 'Copy to clipboard' modal ++y++
- Added Copy info ++y++ ++i++
- Added Copy output ++y++ ++o++
- Copy command and copy preset now copied with quotation marks
- Improved CLI arguments parsing

## 0.8.0 - Help modal, Styling

- Added Help/About modal, ++f1++ / ++question++
- Optimize for network usage (`-movflags faststart`) by default for mp4/mov outputs
- Styling improvements. Internal preparation for theming support
- Improved detecting static images
- Improved filename validations
- ==[Linux]== Fixed clipboard copy warning

## 0.7.2

- Combined Info and Output panes as tabs. Use ++i++ and ++o++ keys for quick switch
- Copy ffmpeg command by pressing ++y++
- Added file overwrite confirmation in 'Render as' modal
- Fixed output format is not considered in 'Render as' modal
- ==[Linux]== Fixed VAAPI initialization. Added `scale_vaapi` filter support
- Added macos builds to CI

## 0.7.1 - Presets

- Added presets ([docs](usage.md#presets)). Press ++p++ to copy preset to clipboard
- Load preset (`--preset ..`) or apply it without displaying TUI (`--preset .. --apply`) to start processing immediately
- ==[Linux]== Specify default VAAPI device `/dev/dri/renderD128` with [ability to override it](params/hardware-acceleration.md) with `EFFY_VAAPI_DEVICE` environment variable
- Added missing validation for height not divisible by 2
- Fixed output file incorrectly constructed in some cases
- **v0.7.1**: switch to a newer clipboard library


## 0.6.0

- Added VAAPI and AMF hardware acceleration options
- Ability to stop running ffmpeg process
- Portrait mode support
- Colorize Info pane items based on media streams
- Hints improvements


## 0.5.0 - Custom values, Output parameter

* Ability to set custom value for parameters by pressing Enter
* Added Output parameter ([docs](https://projects.annimon.com/projects/effy/params/output-format/)). It allows to specify a different output format, for example, extract audio from the given video source
* Adjust precise trim time based on the speed factor
* Better extension defaults for URL inputs


## 0.4.0 - Trim parameter

- Added a Trim parameter that allows to trim media by duration or start/end time
- Added more common Video frame rate values
- Shift+Tab for reverse focus switch


## 0.3.0 - Hardware acceleration, URL input

- Added hardware acceleration parameter. Supports nvidia and intel qsv
- URL input support:
  ```bash
  effy "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
  ```
- 'Render as' dialog redesign with hints


## 0.2.0 - Core rework

* Core rework with different parameters model
* Added 'Save as' modal dialog
* Redraw on resize
