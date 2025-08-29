pub(crate) trait SelectableOption {
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

mod audio_bitrate;
mod disable_audio;
mod video_bitrate;

pub(crate) use audio_bitrate::*;
pub(crate) use disable_audio::*;
pub(crate) use video_bitrate::*;

use crate::{info::Info, model::Param};

pub(crate) fn create_params(info: &Info) -> Vec<(bool, Param)> {
    let mut params = Vec::new();
    if info.has_audio() && info.has_video() {
        params.push((true, Param::DisableAudio(DisableAudio::Off)));
    }
    if info.has_audio() {
        params.push((true, Param::AudioBitrate(AudioBitrate::Auto)));
    }
    if info.has_video() {
        params.push((true, Param::VideoBitrate(VideoBitrate::Auto)));
    }
    params
}

pub(crate) fn recheck_params(params: &mut [(bool, Param)], changed_param: &Param) {
    if let Param::DisableAudio(state) = changed_param {
        params.iter_mut().for_each(|(enabled, param)| {
            if matches!(param, Param::AudioBitrate(_)) {
                *enabled = state == &DisableAudio::Off;
            }
        });
    }
}

pub(crate) fn to_ffmpeg_args(params: Vec<(bool, Param)>) -> Vec<&'static str> {
    let mut args: Vec<&str> = Vec::new();
    for (enabled, param) in &params {
        if !*enabled {
            continue;
        }
        match param {
            Param::DisableAudio(state) => {
                if *state == DisableAudio::On {
                    args.push("-an");
                }
            }
            Param::AudioBitrate(bitrate) => {
                if bitrate != &AudioBitrate::Auto {
                    args.push("-b:a");
                    args.push(bitrate.as_str());
                }
            }
            Param::VideoBitrate(bitrate) => {
                if bitrate != &VideoBitrate::Auto {
                    args.push("-b:v");
                    args.push(bitrate.as_str());
                }
            }
        }
    }
    args
}
