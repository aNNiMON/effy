use std::collections::HashMap;
use std::io::{Error, Read};
use std::process::{Command, Stdio};

use serde::Deserialize;

// https://github.com/FFmpeg/FFmpeg/blob/master/doc/ffprobe.xsd
#[derive(Deserialize, Clone, Debug)]
pub(crate) struct Info {
    pub format: InfoFormat,
    #[serde(default)]
    pub streams: Vec<InfoStream>,
}

#[derive(Deserialize, Clone, Debug)]
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

    pub fn has_video(&self) -> bool {
        self.has_stream_type("video")
    }

    pub fn has_non_empty_duration(&self) -> bool {
        self.get_duration().is_some_and(|dur| dur > 0.0_f64)
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

    pub fn format(&self) -> String {
        let mut r: Vec<String> = Vec::new();
        let format = &self.format;
        r.push(format!("filename: {}", &format.filename));
        r.push(format!("nb_streams: {}", &format.nb_streams));
        macro_rules! format_val {
            ($field:expr, $name:expr) => {
                if let Some(ref v) = $field {
                    r.push(format!("{}: {}", $name, v));
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
                        r.push(format!("{}_{}: {}", stream_type, $name, v));
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
            for (tag, value) in &stream.other {
                match value {
                    serde_json::Value::String(s) => {
                        r.push(format!("{stream_type}_{tag}: {s}"));
                    }
                    serde_json::Value::Number(n) => {
                        r.push(format!("{stream_type}_{tag}: {n}"));
                    }
                    _ => {}
                }
            }
        }

        r.join("\n")
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
