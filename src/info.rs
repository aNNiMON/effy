use std::collections::HashMap;
use std::io::{Error, Read};
use std::process::{Command, Stdio};

use ratatui::text::{Line, Span, Text};
use serde::Deserialize;

use crate::ui::Theme;

// Collect input info using ffprobe

// https://github.com/FFmpeg/FFmpeg/blob/master/doc/ffprobe.xsd
#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Info {
    pub format: InfoFormat,
    #[serde(default)]
    pub streams: Vec<InfoStream>,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub(crate) struct InfoFormat {
    pub filename: String,
    pub nb_streams: u32,
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub(crate) struct InfoStream {
    pub index: u32,
    pub codec_type: Option<String>,
    // video
    pub width: Option<u32>,
    pub height: Option<u32>,
    // audio
    pub sample_rate: Option<String>,
    pub channels: Option<u32>,
    // other
    pub duration: Option<String>,
    pub bit_rate: Option<String>,
    pub max_bit_rate: Option<String>,
    // stream-based
    pub avg_frame_rate: Option<String>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

impl Info {
    pub fn parse(json: &str) -> Result<Self, Error> {
        serde_json::from_str(json).map_err(Error::other)
    }

    pub fn has_audio(&self) -> bool {
        self.has_stream_type("audio")
    }

    pub fn has_audio_only(&self) -> bool {
        self.has_audio() && !self.has_video()
    }

    pub fn has_video(&self) -> bool {
        self.has_stream_type("video")
    }

    pub fn has_non_empty_duration(&self) -> bool {
        self.get_duration().is_some_and(|dur| dur > 0.0_f64) && self.has_more_than_one_frame()
    }

    fn has_more_than_one_frame(&self) -> bool {
        self.streams.iter().any(|s| {
            let fps = s
                .avg_frame_rate
                .as_deref()
                .and_then(|fr| {
                    // avg_frame_rate 25/1
                    let (num, den) = fr.split_once('/')?;
                    let num = num.parse::<f64>().ok()?;
                    let den = den.parse::<f64>().ok()?;
                    if den == 0.0 { None } else { Some(num / den) }
                })
                .unwrap_or(0.0);

            let duration = s
                .duration
                .as_deref()
                .or(self.format.duration.as_deref())
                .and_then(|d| d.parse::<f64>().ok())
                .unwrap_or(0.0);

            duration * fps > 1.0
        })
    }

    pub fn get_duration(&self) -> Option<f64> {
        self.format
            .duration
            .as_deref()
            .and_then(|dur_str| dur_str.parse::<f64>().ok())
    }

    fn has_stream_type(&self, stream_type: &str) -> bool {
        if self.format.nb_streams == 0 {
            false
        } else {
            self.streams
                .iter()
                .any(|s| matches!(&s.codec_type, Some(t) if t == stream_type))
        }
    }

    pub fn format<'a>(&self, theme: &Theme) -> Text<'a> {
        let mut lines = Vec::new();
        let mut add = |k: &str, v: &str, i: u32| {
            let color = theme.color_triplet()[i as usize % 3];
            lines.push(Line::from(vec![
                Span::styled(format!("{k: <24}"), color),
                Span::styled(v.to_owned(), theme.text_param_color()),
            ]));
        };

        let format = &self.format;
        add("filename", &format.filename, 0);
        add("nb_streams", &format.nb_streams.to_string(), 0);
        macro_rules! format_val {
            ($field:expr, $name:expr) => {
                if let Some(ref v) = $field {
                    add($name, v, 0);
                }
            };
        }
        format_val!(format.duration, "duration");
        format_val!(format.size, "size");
        format_val!(format.bit_rate, "bit_rate");

        for stream in &self.streams {
            let index = stream.index;
            let stream_type = format!(
                "{}{}",
                stream
                    .codec_type
                    .as_ref()
                    .and_then(|v| v.get(0..1))
                    .unwrap_or_default(),
                index
            );

            macro_rules! stream_val {
                ($field:expr, $name:expr) => {
                    if let Some(ref v) = $field {
                        let kb = format!("{}_{}", stream_type, $name);
                        add(&kb, &v.to_string(), index + 1);
                    }
                };
            }

            stream_val!(stream.width, "width");
            stream_val!(stream.height, "height");
            stream_val!(stream.sample_rate, "sample_rate");
            stream_val!(stream.channels, "channels");
            stream_val!(stream.duration, "duration");
            stream_val!(stream.bit_rate, "bit_rate");
            stream_val!(stream.max_bit_rate, "max_bit_rate");
            stream_val!(stream.avg_frame_rate, "avg_frame_rate");
            for (tag, value) in &stream.other {
                match value {
                    serde_json::Value::String(s) => {
                        add(&format!("{stream_type}_{tag}"), s, index + 1);
                    }
                    serde_json::Value::Number(n) => {
                        add(&format!("{stream_type}_{tag}"), &n.to_string(), index + 1);
                    }
                    _ => {}
                }
            }
        }

        Text::from(lines)
    }
}

pub(crate) fn get_info(input_file: &str) -> Result<Info, Error> {
    let mut child = Command::new("ffprobe")
        .args([
            "-v",
            "quiet",
            "-of",
            "json=compact=1",
            "-show_format",
            "-show_streams",
        ])
        .arg(input_file)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()?;

    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut output)?;
    }

    let status = child.wait()?;
    if status.success() {
        Info::parse(&output)
    } else {
        Err(Error::other(format!(
            "ffprobe exited with status: {status}"
        )))
    }
}
