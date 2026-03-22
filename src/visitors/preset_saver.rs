use crate::{
    params::*,
    visitors::{PRESET_SEPARATOR, PRESET_VALUE_SEPARATOR, ParameterVisitor, VisitorContext},
};

/// Get preset string from enabled parameters

#[derive(Default)]
pub(crate) struct PresetSaver {
    ctx: VisitorContext,
    preset: Vec<String>,
}

impl PresetSaver {
    pub(crate) fn new(ctx: VisitorContext) -> Self {
        Self {
            ctx,
            preset: Vec::new(),
        }
    }

    pub fn collect(&self) -> String {
        self.preset.join(PRESET_SEPARATOR)
    }

    fn add(&mut self, id: &str, value: &str) {
        self.preset
            .push(format!("{id}{PRESET_VALUE_SEPARATOR}{value}"));
    }
}

impl ParameterVisitor for PresetSaver {
    fn visit_trim(&mut self, data: &mut ParameterData) {
        if let Some(v) = Trim::save_preset(&self.ctx, data) {
            self.add(Trim::ID, &v);
        }
    }

    fn visit_disable_audio(&mut self, data: &mut ParameterData) {
        if let Some(v) = DisableAudio::save_preset(&self.ctx, data) {
            self.add(DisableAudio::ID, &v);
        }
    }

    fn visit_audio_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioBitrate::save_preset(&self.ctx, data) {
            self.add(AudioBitrate::ID, &v);
        }
    }

    fn visit_audio_crystalizer(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioCrystalizer::save_preset(&self.ctx, data) {
            self.add(AudioCrystalizer::ID, &v);
        }
    }

    fn visit_audio_volume(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioVolume::save_preset(&self.ctx, data) {
            self.add(AudioVolume::ID, &v);
        }
    }

    fn visit_audio_pitch(&mut self, data: &mut ParameterData) {
        if let Some(v) = AudioPitch::save_preset(&self.ctx, data) {
            self.add(AudioPitch::ID, &v);
        }
    }

    fn visit_speed_factor(&mut self, data: &mut ParameterData) {
        if let Some(v) = SpeedFactor::save_preset(&self.ctx, data) {
            self.add(SpeedFactor::ID, &v);
        }
    }

    fn visit_video_bitrate(&mut self, data: &mut ParameterData) {
        if let Some(v) = VideoBitrate::save_preset(&self.ctx, data) {
            self.add(VideoBitrate::ID, &v);
        }
    }

    fn visit_video_frame_rate(&mut self, data: &mut ParameterData) {
        if let Some(v) = VideoFrameRate::save_preset(&self.ctx, data) {
            self.add(VideoFrameRate::ID, &v);
        }
    }

    fn visit_video_scale(&mut self, data: &mut ParameterData) {
        if let Some(v) = VideoScale::save_preset(&self.ctx, data) {
            self.add(VideoScale::ID, &v);
        }
    }

    fn visit_hardware_acceleration(&mut self, data: &mut ParameterData) {
        if let Some(v) = HardwareAcceleration::save_preset(&self.ctx, data) {
            self.add(HardwareAcceleration::ID, &v);
        }
    }

    fn visit_output_format(&mut self, data: &mut ParameterData) {
        if let Some(v) = OutputFormat::save_preset(&self.ctx, data) {
            self.add(OutputFormat::ID, &v);
        }
    }

    fn visit_last(&mut self) {}
}
