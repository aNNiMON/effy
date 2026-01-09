mod macros;
mod parameter;

mod audio_bitrate;
mod audio_crystalizer;
mod audio_pitch;
mod audio_volume;
mod disable_audio;
mod hardware_acceleration;
mod output_format;
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
pub(crate) use output_format::*;
pub(crate) use parameter::{Parameter, ParameterData, PresetParameter, SelectOption};
pub(crate) use speed_factor::*;
pub(crate) use trim::*;
pub(crate) use video_bitrate::*;
pub(crate) use video_frame_rate::*;
pub(crate) use video_scale::*;

use crate::{
    info::Info,
    params::macros::select_option,
    visitors::{FFmpegParameterVisitor, PresetApplier},
};
use std::collections::HashMap;

pub(crate) fn create_params(info: &Info, preset: Option<&str>, source_ext: &str) -> Vec<Parameter> {
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
    params.push(OutputFormat::new_parameter(info, source_ext));
    if let Some(preset_value) = preset {
        apply_preset(&mut params, preset_value);
    }
    recheck_params(&mut params);
    params
}

pub(crate) fn apply_preset(params: &mut [Parameter], preset: &str) {
    let preset_map = preset
        .split(';')
        .filter_map(|p| p.split_once(':'))
        .collect::<HashMap<&str, &str>>();
    let mut preset_applier = PresetApplier::new(preset_map);
    apply_visitor(&mut preset_applier, params);
}

pub(crate) fn recheck_params(params: &mut [Parameter]) {
    let result_is_audio = if let Some(result_format) = params
        .iter()
        .filter(|param| param.id == OutputFormat::ID)
        .find_map(|param| select_option!(&param.data))
    {
        OutputFormat::is_audio(&result_format.value)
    } else {
        false
    };

    let audio_is_disabled = if let Some(disable_audio) = params
        .iter()
        .filter(|param| param.id == DisableAudio::ID)
        .find_map(|param| match &param.data {
            ParameterData::Toggle { value } => Some(value),
            _ => None,
        }) {
        !result_is_audio && *disable_audio
    } else {
        false
    };

    for param in params {
        if matches!(
            param.id,
            DisableAudio::ID
                | VideoScale::ID
                | VideoBitrate::ID
                | VideoFrameRate::ID
                | HardwareAcceleration::ID
        ) {
            param.enabled = !result_is_audio;
        }

        if matches!(
            param.id,
            AudioBitrate::ID | AudioCrystalizer::ID | AudioPitch::ID | AudioVolume::ID
        ) {
            param.enabled = !audio_is_disabled;
        }
    }
}

pub(crate) fn apply_visitor(visitor: &mut dyn FFmpegParameterVisitor, params: &mut [Parameter]) {
    let mut sorted_params: Vec<&mut Parameter> =
        params.iter_mut().filter(|param| param.enabled).collect();
    sorted_params.sort_by_key(|param| param.order);
    for param in sorted_params {
        match param.id {
            Trim::ID => visitor.visit_trim(&mut param.data),
            DisableAudio::ID => visitor.visit_disable_audio(&mut param.data),
            AudioVolume::ID => visitor.visit_audio_volume(&mut param.data),
            AudioBitrate::ID => visitor.visit_audio_bitrate(&mut param.data),
            AudioCrystalizer::ID => visitor.visit_audio_crystalizer(&mut param.data),
            AudioPitch::ID => visitor.visit_audio_pitch(&mut param.data),
            SpeedFactor::ID => visitor.visit_speed_factor(&mut param.data),
            VideoBitrate::ID => visitor.visit_video_bitrate(&mut param.data),
            VideoFrameRate::ID => visitor.visit_video_frame_rate(&mut param.data),
            VideoScale::ID => visitor.visit_video_scale(&mut param.data),
            HardwareAcceleration::ID => visitor.visit_hardware_acceleration(&mut param.data),
            OutputFormat::ID => visitor.visit_output_format(&mut param.data),
            _ => {}
        }
    }
}
