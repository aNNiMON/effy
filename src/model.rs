use crate::params::*;

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
    AudioCrystalizer(AudioCrystalizer),
    AudioVolume(AudioVolume),
    AudioPitch(AudioPitch),
    SpeedFactor(SpeedFactor),
    VideoBitrate(VideoBitrate),
    VideoFrameRate(VideoFrameRate),
    VideoScale(VideoScale),
}

impl Param {
    pub(crate) fn toggle_prev(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(SelectableOption::toggle_prev(val)),
            Param::AudioBitrate(v) => Param::AudioBitrate(SelectableOption::toggle_prev(v)),
            Param::AudioCrystalizer(v) => Param::AudioCrystalizer(SelectableOption::toggle_prev(v)),
            Param::AudioVolume(volume) => Param::AudioVolume(SelectableOption::toggle_prev(volume)),
            Param::AudioPitch(pitch) => Param::AudioPitch(SelectableOption::toggle_prev(pitch)),
            Param::SpeedFactor(speed) => Param::SpeedFactor(SelectableOption::toggle_prev(speed)),
            Param::VideoBitrate(v) => Param::VideoBitrate(SelectableOption::toggle_prev(v)),
            Param::VideoFrameRate(fps) => Param::VideoFrameRate(SelectableOption::toggle_prev(fps)),
            Param::VideoScale(scale) => Param::VideoScale(SelectableOption::toggle_prev(scale)),
        }
    }

    pub(crate) fn toggle_next(&self) -> Self {
        match self {
            Param::DisableAudio(val) => Param::DisableAudio(SelectableOption::toggle_next(val)),
            Param::AudioBitrate(v) => Param::AudioBitrate(SelectableOption::toggle_next(v)),
            Param::AudioCrystalizer(v) => Param::AudioCrystalizer(SelectableOption::toggle_next(v)),
            Param::AudioVolume(volume) => Param::AudioVolume(SelectableOption::toggle_next(volume)),
            Param::AudioPitch(pitch) => Param::AudioPitch(SelectableOption::toggle_next(pitch)),
            Param::SpeedFactor(speed) => Param::SpeedFactor(SelectableOption::toggle_next(speed)),
            Param::VideoBitrate(v) => Param::VideoBitrate(SelectableOption::toggle_next(v)),
            Param::VideoFrameRate(fps) => Param::VideoFrameRate(SelectableOption::toggle_next(fps)),
            Param::VideoScale(scale) => Param::VideoScale(SelectableOption::toggle_next(scale)),
        }
    }

    pub(crate) fn as_str(&self) -> String {
        match self {
            Param::DisableAudio(val) => val.as_str(),
            Param::AudioBitrate(bitrate) => bitrate.as_str(),
            Param::AudioCrystalizer(cr) => cr.as_str(),
            Param::AudioVolume(volume) => volume.as_str(),
            Param::AudioPitch(pitch) => pitch.as_str(),
            Param::SpeedFactor(speed) => speed.as_str(),
            Param::VideoBitrate(bitrate) => bitrate.as_str(),
            Param::VideoFrameRate(fps) => fps.as_str(),
            Param::VideoScale(scale) => scale.as_str(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        match self {
            Param::DisableAudio(val) => val.describe_self(),
            Param::AudioBitrate(bitrate) => bitrate.describe_self(),
            Param::AudioCrystalizer(cr) => cr.describe_self(),
            Param::AudioVolume(volume) => volume.describe_self(),
            Param::AudioPitch(pitch) => pitch.describe_self(),
            Param::SpeedFactor(speed) => speed.describe_self(),
            Param::VideoBitrate(bitrate) => bitrate.describe_self(),
            Param::VideoFrameRate(fps) => fps.describe_self(),
            Param::VideoScale(scale) => scale.describe_self(),
        }
    }

    pub(crate) fn describe(&self) -> String {
        format!("{}: {}", self.get_name(), self.as_str())
    }
}

pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
    SaveCompleted(bool),
}
