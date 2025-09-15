pub(crate) trait SelectableOption: 'static + Sized + PartialEq + Clone {
    fn variants() -> &'static [Self];
    fn variant_index(&self) -> Option<usize> {
        Self::variants().iter().position(|v| *v == *self)
    }

    fn as_str(&self) -> String;
    fn describe_self(&self) -> &'static str;

    fn toggle_prev(&self) -> Self {
        let variants = Self::variants();
        let len = variants.len();
        let idx = if let Some(i) = self.variant_index() {
            if i == 0 { len - 1 } else { i - 1 }
        } else {
            0
        };
        variants[idx].clone()
    }

    fn toggle_next(&self) -> Self {
        let variants = Self::variants();
        let idx = if let Some(i) = self.variant_index()
            && i < variants.len() - 1
        {
            i + 1
        } else {
            0
        };
        variants[idx].clone()
    }
}

mod audio_bitrate;
mod audio_crystalizer;
mod audio_pitch;
mod audio_volume;
mod disable_audio;
mod macros;
mod speed_factor;
mod video_bitrate;
mod video_frame_rate;
mod video_scale;

pub(crate) use audio_bitrate::*;
pub(crate) use audio_crystalizer::*;
pub(crate) use audio_pitch::*;
pub(crate) use audio_volume::*;
pub(crate) use disable_audio::*;
pub(crate) use speed_factor::*;
pub(crate) use video_bitrate::*;
pub(crate) use video_frame_rate::*;
pub(crate) use video_scale::*;

use crate::{
    info::Info,
    model::Param,
    visitors::{FFmpegParameter, FFmpegParameterVisitor},
};

pub(crate) fn create_params(info: &Info) -> Vec<(bool, Param)> {
    let mut params = Vec::new();
    if info.has_audio() && info.has_video() {
        params.push((true, Param::DisableAudio(DisableAudio::default())));
    }
    if info.has_audio() {
        params.push((true, Param::AudioBitrate(AudioBitrate::default())));
        params.push((true, Param::AudioCrystalizer(AudioCrystalizer::default())));
        params.push((true, Param::AudioVolume(AudioVolume::default())));
        params.push((true, Param::AudioPitch(AudioPitch::default())));
    }
    params.push((true, Param::SpeedFactor(SpeedFactor::default())));
    if info.has_video() {
        params.push((true, Param::VideoBitrate(VideoBitrate::default())));
        params.push((true, Param::VideoFrameRate(VideoFrameRate::default())));
        params.push((true, Param::VideoScale(VideoScale::default())));
    }
    params
}

pub(crate) fn recheck_params(params: &mut [(bool, Param)], changed_param: &Param) {
    if let Param::DisableAudio(state) = changed_param {
        params.iter_mut().for_each(|(enabled, param)| {
            let audio_enabled = state.value == DisableAudio::OFF;
            if matches!(
                param,
                Param::AudioBitrate(_)
                    | Param::AudioCrystalizer(_)
                    | Param::AudioVolume(_)
                    | Param::AudioPitch(_)
            ) {
                *enabled = audio_enabled;
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
                Param::AudioCrystalizer(p) => p.accept(visitor),
                Param::AudioVolume(p) => p.accept(visitor),
                Param::AudioPitch(p) => p.accept(visitor),
                Param::SpeedFactor(p) => p.accept(visitor),
                Param::VideoBitrate(p) => p.accept(visitor),
                Param::VideoFrameRate(p) => p.accept(visitor),
                Param::VideoScale(p) => p.accept(visitor),
            }
        }
    }
}
