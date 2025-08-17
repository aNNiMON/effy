use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read};
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
pub struct InfoFormat {
    pub filename: String,
    pub nb_streams: u32,
    pub duration: Option<String>,
    pub size: Option<String>,
    pub bit_rate: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct InfoStream {
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
    pub fn parse(json: &str) -> Result<String, Error> {
        let data: Info = serde_json::from_str(json).map_err(|e| Error::new(ErrorKind::Other, e))?;
        Ok(data.format())
    }

    pub fn format(&self) -> String {
        let mut r: Vec<String> = Vec::new();
        let format = &self.format;
        r.push(format!("filename: {}", format.filename));
        r.push(format!("nb_streams: {}", format.nb_streams));
        if let Some(v) = format.duration.clone() {
            r.push(format!("duration: {}", v));
        }
        if let Some(v) = format.size.clone() {
            r.push(format!("size: {}", v));
        }
        if let Some(v) = format.bit_rate.clone() {
            r.push(format!("bitrate: {}", v));
        }
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
            if let Some(v) = stream.width.clone() {
                r.push(format!("{}_{}: {}", stream_type, "width", v));
            }
            if let Some(v) = stream.height.clone() {
                r.push(format!("{}_{}: {}", stream_type, "height", v));
            }
            if let Some(v) = stream.sample_rate.clone() {
                r.push(format!("{}_{}: {}", stream_type, "sample_rate", v));
            }
            if let Some(v) = stream.channels.clone() {
                r.push(format!("{}_{}: {}", stream_type, "channels", v));
            }
            if let Some(v) = stream.duration.clone() {
                r.push(format!("{}_{}: {}", stream_type, "duration", v));
            }
            if let Some(v) = stream.bit_rate.clone() {
                r.push(format!("{}_{}: {}", stream_type, "bit_rate", v));
            }
            if let Some(v) = stream.max_bit_rate.clone() {
                r.push(format!("{}_{}: {}", stream_type, "max_bit_rate", v));
            }
            for (tag, value) in &stream.other {
                match value {
                    serde_json::Value::String(s) => {
                        r.push(format!("{}_{}: {}", stream_type, tag, s));
                    }
                    serde_json::Value::Number(n) => {
                        r.push(format!("{}_{}: {}", stream_type, tag, n));
                    }
                    _ => {}
                }
            }
        }

        r.join("\n")
    }
}

pub(crate) fn get_info(input_file: String) -> Result<String, Error> {
    let mut child = match Command::new("ffprobe")
        .args(&[
            "-v",
            "quiet",
            "-of",
            "json=compact=1",
            "-show_format",
            "-show_streams",
        ])
        .arg(&input_file)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => return Err(e.into()),
    };

    let mut output = String::new();
    if let Some(mut stdout) = child.stdout.take() {
        stdout.read_to_string(&mut output)?;
    }

    let status = child.wait()?;
    if status.success() {
        Info::parse(&output)
    } else {
        Err(Error::new(
            ErrorKind::Other,
            format!("ffprobe exited with status: {}", status),
        ))
    }
}
