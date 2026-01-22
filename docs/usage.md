---
icon: lucide/book-open
---

# Usage

!!! info "Important"

    `ffmpeg` and `ffprobe` must be [installed](https://ffmpeg.org/download.html) and available in the `PATH` environment variable.

effy supports local sources:

```bash
effy input.mp4
```

and remote sources:

```bash
effy "https://commondatastorage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4"
```

----

## CLI

In addition to the required **input** argument, which should always be specified last, effy has the following command-line arguments:

- `--preset <preset>` specifies a set of preconfigured parameter values.
- `--apply` applicable only if **preset** is specified. It starts processing the **input** in CLI mode immediately without displaying the TUI.

### Presets

Preset is a semicolon-separated list of parameters with their values that will be preconfigured once TUI is displayed.

Example:

```bash
effy --preset "noaudio:1;vbitrate:90k;scale:400;output:mp4" input.mp4
```

If the `--apply` argument is specified, the TUI won't be displayed and processing will start immediately.

----

## TUI

### Terminal window

effy has 3 selectable panes: parameters, info and output. The Parameters pane is focused by default. Press ++tab++ or ++shift+tab++ to switch focus.

#### Parameters pane

This pane contains a list of parameters to change based on the type of source provided. The list is dynamically constructed and parameters are automatically disabled when they can't be applied.

#### Info pane

Contains general information about the provided source: file name, duration, number of streams and their types, bit rate, codecs, dimensions, etc. Basically, it's a structured `ffprobe` output in one pane.

Use ++i++ to quickly switch to this pane.

#### Output pane

When rendering starts (++s++ or ++ctrl+s++) the FFmpeg processing output redirects to this pane.

To stop processing, for example, after realizing that a parameter was set incorrectly, hit ++q++ or ++Esc++ while media is being rendered. This sends a stop signal to `ffmpeg`.

Use ++o++ to quickly switch to this pane.

## Portrait mode

If terminal width is not wide enough to fit Params and Output panes, effy switches to a Portrait mode. In this mode all panes arranged vertically and modals horizontal spacing is removed.
