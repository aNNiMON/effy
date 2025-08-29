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

use crate::{
    info::Info,
    model::Param,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

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

pub(crate) fn apply_visitor(visitor: &mut dyn FFmpegParameterVisitor, params: Vec<(bool, Param)>) {
    for (enabled, param) in &params {
        if *enabled {
            match param {
                Param::DisableAudio(p) => p.accept(visitor),
                Param::AudioBitrate(p) => p.accept(visitor),
                Param::VideoBitrate(p) => p.accept(visitor),
            }
        }
    }
}
