use crate::params::{AudioBitrate, DisableAudio, SelectableOption, VideoBitrate};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Param {
    DisableAudio(DisableAudio),
    AudioBitrate(AudioBitrate),
    VideoBitrate(VideoBitrate),
}

impl Param {
    pub(crate) fn toggle_prev(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(val.toggle_prev()),
            Param::AudioBitrate(bitrate) => Param::AudioBitrate(bitrate.toggle_prev()),
            Param::VideoBitrate(bitrate) => Param::VideoBitrate(bitrate.toggle_prev()),
        }
    }

    pub(crate) fn toggle_next(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(val.toggle_next()),
            Param::AudioBitrate(bitrate) => Param::AudioBitrate(bitrate.toggle_next()),
            Param::VideoBitrate(bitrate) => Param::VideoBitrate(bitrate.toggle_next()),
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Param::DisableAudio(val) => val.as_str(),
            Param::AudioBitrate(bitrate) => bitrate.as_str(),
            Param::VideoBitrate(bitrate) => bitrate.as_str(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        match self {
            Param::DisableAudio(val) => val.describe_self(),
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
