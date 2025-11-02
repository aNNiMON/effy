mod macros;
mod parameter;

mod audio_bitrate;
mod audio_crystalizer;
mod audio_pitch;
mod audio_volume;
mod disable_audio;
mod hardware_acceleration;
mod speed_factor;
mod trim;
mod video_bitrate;
mod video_frame_rate;
mod video_scale;

pub(crate) use audio_bitrate::*;
pub(crate) use audio_crystalizer::*;
pub(crate) use audio_pitch::*;
pub(crate) use audio_volume::*;
pub(crate) use disable_audio::*;
pub(crate) use hardware_acceleration::*;
pub(crate) use parameter::{Parameter, ParameterData, SelectOption};
pub(crate) use speed_factor::*;
pub(crate) use trim::*;
pub(crate) use video_bitrate::*;
pub(crate) use video_frame_rate::*;
pub(crate) use video_scale::*;

use crate::{info::Info, visitors::FFmpegParameterVisitor};

pub(crate) fn create_params(info: &Info) -> Vec<Parameter> {
    let mut params: Vec<Parameter> = Vec::new();
    if info.has_non_empty_duration() {
        params.push(Trim::new_parameter());
    }
    if info.has_audio() && info.has_video() {
        params.push(DisableAudio::new_parameter());
    }
    if info.has_audio() {
        params.push(AudioBitrate::new_parameter());
        params.push(AudioCrystalizer::new_parameter());
        params.push(AudioVolume::new_parameter());
        params.push(AudioPitch::new_parameter());
    }
    params.push(SpeedFactor::new_parameter());
    if info.has_video() {
        params.push(VideoBitrate::new_parameter());
        params.push(VideoFrameRate::new_parameter());
        params.push(VideoScale::new_parameter());
        params.push(HardwareAcceleration::new_parameter());
    }
    params
}

pub(crate) fn recheck_params(params: &mut [Parameter], changed_param: &Parameter) {
    if let Parameter {
        id: DisableAudio::ID,
        data: ParameterData::Toggle { value },
        ..
    } = changed_param
    {
        for param in params {
            if matches!(
                param.id,
                AudioBitrate::ID | AudioCrystalizer::ID | AudioPitch::ID | AudioVolume::ID
            ) {
                param.enabled = !*value;
            }
        }
    }
}

pub(crate) fn apply_visitor(visitor: &mut dyn FFmpegParameterVisitor, params: &[Parameter]) {
    for param in params {
        if !param.enabled {
            continue;
        }
        match param.id {
            Trim::ID => visitor.visit_trim(&param.data),
            DisableAudio::ID => visitor.visit_disable_audio(&param.data),
            AudioVolume::ID => visitor.visit_audio_volume(&param.data),
            AudioBitrate::ID => visitor.visit_audio_bitrate(&param.data),
            AudioCrystalizer::ID => visitor.visit_audio_crystalizer(&param.data),
            AudioPitch::ID => visitor.visit_audio_pitch(&param.data),
            SpeedFactor::ID => visitor.visit_speed_factor(&param.data),
            VideoBitrate::NAME => visitor.visit_video_bitrate(&param.data),
            VideoFrameRate::ID => visitor.visit_video_frame_rate(&param.data),
            VideoScale::ID => visitor.visit_video_scale(&param.data),
            HardwareAcceleration::ID => visitor.visit_hardware_acceleration(&param.data),
            _ => {}
        }
    }
}
