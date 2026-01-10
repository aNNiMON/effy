use std::collections::HashMap;

use crate::{params::*, visitors::FFmpegParameterVisitor};

#[derive(Default)]
pub(crate) struct PresetApplier<'a> {
    preset_map: HashMap<&'a str, &'a str>,
}

impl<'a> PresetApplier<'a> {
    pub(crate) fn new(preset: &'a str) -> Self {
        let preset_map = preset
            .split(';')
            .filter_map(|p| p.split_once(':'))
            .collect::<HashMap<&str, &str>>();
        Self { preset_map }
    }
}

impl FFmpegParameterVisitor for PresetApplier<'_> {
    fn visit_trim(&mut self, _data: &mut ParameterData) {}

    fn visit_disable_audio(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(DisableAudio::ID) {
            DisableAudio::apply_preset(data, preset_value);
        }
    }

    fn visit_audio_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioBitrate::ID) {
            AudioBitrate::apply_preset(data, preset_value);
        }
    }

    fn visit_audio_crystalizer(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioCrystalizer::ID) {
            AudioCrystalizer::apply_preset(data, preset_value);
        }
    }

    fn visit_audio_volume(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioVolume::ID) {
            AudioVolume::apply_preset(data, preset_value);
        }
    }

    fn visit_audio_pitch(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(AudioPitch::ID) {
            AudioPitch::apply_preset(data, preset_value);
        }
    }

    fn visit_speed_factor(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(SpeedFactor::ID) {
            SpeedFactor::apply_preset(data, preset_value);
        }
    }

    fn visit_video_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(VideoBitrate::ID) {
            VideoBitrate::apply_preset(data, preset_value);
        }
    }

    fn visit_video_frame_rate(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(VideoFrameRate::ID) {
            VideoFrameRate::apply_preset(data, preset_value);
        }
    }

    fn visit_video_scale(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(VideoScale::ID) {
            VideoScale::apply_preset(data, preset_value);
        }
    }

    fn visit_hardware_acceleration(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(HardwareAcceleration::ID) {
            HardwareAcceleration::apply_preset(data, preset_value);
        }
    }

    fn visit_output_format(&mut self, data: &mut ParameterData) {
        if let Some(preset_value) = self.preset_map.get(OutputFormat::ID) {
            OutputFormat::apply_preset(data, preset_value);
        }
    }
}
