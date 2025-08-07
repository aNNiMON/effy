#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Config,
}

pub(crate) trait Parameter {
    fn toggle_prev(&self) -> Self;
    fn toggle_next(&self) -> Self;

    fn as_str(&self) -> &'static str;
    fn describe(&self) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioBitrate {
    K32,
    K80,
    K128,
    K192,
    K256,
    Auto,
}

impl Parameter for AudioBitrate {
    fn toggle_prev(&self) -> Self {
        match self {
            AudioBitrate::K32 => AudioBitrate::Auto,
            AudioBitrate::K80 => AudioBitrate::K32,
            AudioBitrate::K128 => AudioBitrate::K80,
            AudioBitrate::K192 => AudioBitrate::K128,
            AudioBitrate::K256 => AudioBitrate::K192,
            AudioBitrate::Auto => AudioBitrate::K256,
        }
    }

    fn toggle_next(&self) -> Self {
        match self {
            AudioBitrate::K32 => AudioBitrate::K80,
            AudioBitrate::K80 => AudioBitrate::K128,
            AudioBitrate::K128 => AudioBitrate::K192,
            AudioBitrate::K192 => AudioBitrate::K256,
            AudioBitrate::K256 => AudioBitrate::Auto,
            AudioBitrate::Auto => AudioBitrate::K32,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            AudioBitrate::K32 => "32k",
            AudioBitrate::K80 => "80k",
            AudioBitrate::K128 => "128k",
            AudioBitrate::K192 => "192k",
            AudioBitrate::K256 => "256k",
            AudioBitrate::Auto => "auto",
        }
    }

    fn describe(&self) -> String {
        format!("Audio Bitrate: {}", self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoBitrate {
    K768,
    M1,
    M2,
    M4,
    Auto,
}

impl Parameter for VideoBitrate {
    fn toggle_next(&self) -> Self {
        match self {
            VideoBitrate::K768 => VideoBitrate::M1,
            VideoBitrate::M1 => VideoBitrate::M2,
            VideoBitrate::M2 => VideoBitrate::M4,
            VideoBitrate::M4 => VideoBitrate::Auto,
            VideoBitrate::Auto => VideoBitrate::K768,
        }
    }

    fn toggle_prev(&self) -> Self {
        match self {
            VideoBitrate::K768 => VideoBitrate::Auto,
            VideoBitrate::M1 => VideoBitrate::K768,
            VideoBitrate::M2 => VideoBitrate::M1,
            VideoBitrate::M4 => VideoBitrate::M2,
            VideoBitrate::Auto => VideoBitrate::M4,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            VideoBitrate::K768 => "768k",
            VideoBitrate::M1 => "1M",
            VideoBitrate::M2 => "2M",
            VideoBitrate::M4 => "4M",
            VideoBitrate::Auto => "auto",
        }
    }

    fn describe(&self) -> String {
        format!("Video Bitrate: {}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Param {
    DisableAudio(bool),
    AudioBitrate(AudioBitrate),
    VideoBitrate(VideoBitrate),
}

impl Parameter for Param {
    fn toggle_prev(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(!val),
            Param::AudioBitrate(bitrate) => Param::AudioBitrate(bitrate.toggle_prev()),
            Param::VideoBitrate(bitrate) => Param::VideoBitrate(bitrate.toggle_prev()),
        }
    }

    fn toggle_next(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(!val),
            Param::AudioBitrate(bitrate) => Param::AudioBitrate(bitrate.toggle_next()),
            Param::VideoBitrate(bitrate) => Param::VideoBitrate(bitrate.toggle_next()),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Param::DisableAudio(val) => {
                if *val {
                    "on"
                } else {
                    "off"
                }
            }
            Param::AudioBitrate(bitrate) => bitrate.as_str(),
            Param::VideoBitrate(bitrate) => bitrate.as_str(),
        }
    }

    fn describe(&self) -> String {
        match self {
            Param::DisableAudio(val) => format!("Disable Audio: {}", self.as_str()),
            Param::AudioBitrate(bitrate) => bitrate.describe(),
            Param::VideoBitrate(bitrate) => bitrate.describe(),
        }
    }
}
