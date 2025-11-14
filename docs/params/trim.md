---
icon: lucide/scissors
tags:
  - audio
  - video
---

# Trim

Allows to trim (cut) audio or video.

![trim modal](../assets/trim.png#only-dark)
![trim modal](../assets/trimw.png#only-light)

## Inputs

 - ==Start==: the time from which the trim should start
 - ==To==: the time by which the trim should be finished
 - ==Duration==: the duration from the ==Start== time
 - ==Precise==: if disabled, the trim will be performed using the nearest keyframes. This method is faster, but inaccurate. If enabled, the trim will be precise, but slower
 - ==Use Duration/To==: switch between the ==To== or ==Duration== input

## Value format

Accepts either the seconds or the timestamp format.

### Seconds format

Accepts a positive decimal value in seconds. Examples: `5`, `10`, `20.45`, `0.8`

### Timestamp format

Accepts a timestamp in any of the following forms:

 - `HH:MM:SS.mmm` - full form
 - `HH:MM:SS` - without milliseconds
 - `MM:SS.mmm` - without hours
 - `MM:SS` - minutes and seconds only

All component values can be optionally zero-padded.

Component | Required? | Description | Range | Example
----------|-----------|-------------|-------|--------
`HH` | :lucide-square: | Hours | 0-99 | `00`, `0`, `01`, `1`
`MM` | :lucide-square-check: |  Minutes | 0-59 | `00`, `0`, `5`, `05`, `35`
`SS` | :lucide-square-check: | Seconds | 0-59 | `00`, `0`, `7`, `07`, `57`
`mmm` | :lucide-square: | Milliseconds | 0-999 | `000`, `0`, `123`, `50`

