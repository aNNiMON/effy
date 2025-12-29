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


## Terminal window

effy has 3 selectable panes: info, parameters, and output. The Parameters pane is focused by default. Press ++tab++ or ++shift+tab++ to switch focus.

### Info pane

Contains general information about the provided source: file name, duration, number of streams and their types, bit rate, codecs, dimensions, etc. Basically, it's a structured `ffprobe` output in one pane.

### Parameters pane

This pane contains a list of parameters to change based on the type of source provided. The list is dynamically constructed and parameters are automatically disabled when they can't be applied.

### Output pane

When rendering starts (++s++ or ++ctrl+s++) the FFmpeg processing output redirects to this pane.

To stop processing, for example, after realizing that a parameter was set incorrectly, hit ++q++ or ++Esc++ while media is being rendered. This sends a stop signal to `ffmpeg`.


## Portrait mode

If terminal width is not wide enough to fit Params and Output panes, effy switches to a Portrait mode. In this mode all panes arranged vertically and models horizontal spacing is removed.
