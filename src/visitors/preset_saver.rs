use crate::{params::*, visitors::FFmpegParameterVisitor};

#[derive(Default)]
pub(crate) struct PresetSaver {
    preset: Vec<String>,
}

impl PresetSaver {
    pub(crate) fn new() -> Self {
        Self { preset: Vec::new() }
    }

    pub fn collect(&self) -> String {
        self.preset.join(";")
    }

    fn add(&mut self, id: &str, value: &str) {
        self.preset.push(format!("{}:{}", id, value));
    }
}

impl FFmpegParameterVisitor for PresetSaver {
    fn visit_trim(&mut self, _data: &mut ParameterData) {}

    fn visit_disable_audio(&mut self, data: &mut ParameterData) {
        if let Some(v) = DisableAudio::save_preset(data) {
            self.add(DisableAudio::ID, v);
        }
    }

    fn visit_audio_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioBitrate::save_preset(data) {
            self.add(AudioBitrate::ID, v);
        }
    }

    fn visit_audio_crystalizer(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioCrystalizer::save_preset(data) {
            self.add(AudioCrystalizer::ID, v);
        }
    }

    fn visit_audio_volume(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioVolume::save_preset(data) {
            self.add(AudioVolume::ID, v);
        }
    }

    fn visit_audio_pitch(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioPitch::save_preset(data) {
            self.add(AudioPitch::ID, v);
        }
    }

    fn visit_speed_factor(&mut self, data: &mut ParameterData) {
        if let Some(v) = SpeedFactor::save_preset(data) {
            self.add(SpeedFactor::ID, v);
        }
    }

    fn visit_video_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(v) = VideoBitrate::save_preset(data) {
            self.add(VideoBitrate::ID, v);
        }
    }

    fn visit_video_frame_rate(&mut self, data: &mut ParameterData) {
        if let Some(v) = VideoFrameRate::save_preset(data) {
            self.add(VideoFrameRate::ID, v);
        }
    }

    fn visit_video_scale(&mut self, data: &mut ParameterData) {
        if let Some(v) = VideoScale::save_preset(data) {
            self.add(VideoScale::ID, v);
        }
    }

    fn visit_hardware_acceleration(&mut self, data: &mut ParameterData) {
        if let Some(v) = HardwareAcceleration::save_preset(data) {
            self.add(HardwareAcceleration::ID, v);
        }
    }

    fn visit_output_format(&mut self, data: &mut ParameterData) {
        if let Some(v) = OutputFormat::save_preset(data) {
            self.add(OutputFormat::ID, v);
        }
    }
}
