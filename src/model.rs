use strum::VariantArray;
use strum_macros::VariantArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Output,
}

pub(crate) trait Parameter {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized;

    fn toggle_prev(&self) -> Self
    where
        Self: 'static + Sized + PartialEq + Clone,
    {
        self.variants()
            .iter()
            .cycle()
            .take_while(|&v| *v != *self)
            .last()
            .cloned()
            .unwrap_or_else(|| self.variants()[self.variants().len() - 1].clone())
    }

    fn toggle_next(&self) -> Self
    where
        Self: 'static + Sized + PartialEq + Clone,
    {
        self.variants()
            .iter()
            .cycle()
            .skip_while(|&v| *v != *self)
            .nth(1)
            .cloned()
            .unwrap_or_else(|| self.variants()[0].clone())
    }

    fn as_str(&self) -> &'static str;
    fn describe_self(&self) -> &'static str;
}

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioBitrate {
    K32,
    K80,
    K128,
    K192,
    K256,
    Auto,
}

impl Parameter for AudioBitrate {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        AudioBitrate::VARIANTS
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

    fn describe_self(&self) -> &'static str {
        "Audio Bitrate"
    }
}

#[derive(Debug, VariantArray, Clone, Copy, PartialEq, Eq)]
pub(crate) enum VideoBitrate {
    K768,
    M1,
    M2,
    M4,
    Auto,
}

impl Parameter for VideoBitrate {
    fn variants(&self) -> &'static [Self]
    where
        Self: Sized,
    {
        VideoBitrate::VARIANTS
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

    fn describe_self(&self) -> &'static str {
        "Video Bitrate"
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Param {
    DisableAudio(bool),
    AudioBitrate(AudioBitrate),
    VideoBitrate(VideoBitrate),
}

impl Param {
    pub(crate) fn toggle_prev(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(!val),
            Param::AudioBitrate(bitrate) => Param::AudioBitrate(bitrate.toggle_prev()),
            Param::VideoBitrate(bitrate) => Param::VideoBitrate(bitrate.toggle_prev()),
        }
    }

    pub(crate) fn toggle_next(&self) -> Self {
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

    pub(crate) fn get_name(&self) -> &str {
        match self {
            Param::DisableAudio(_) => "Disable Audio",
            Param::AudioBitrate(bitrate) => bitrate.describe_self(),
            Param::VideoBitrate(bitrate) => bitrate.describe_self(),
        }
    }

    pub(crate) fn describe(&self) -> String {
        format!("{}: {}", self.get_name(), self.as_str())
    }
}

pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
}
