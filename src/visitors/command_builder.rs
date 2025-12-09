use crate::{params::*, visitors::FFmpegParameterVisitor};

#[derive(Default)]
pub(crate) struct CommandBuilder {
    pub(crate) discard_audio: bool,
    pub(crate) hwaccel: HWAccel,
    pub(crate) speed_factor: Option<f64>,
    pub(crate) audio_filters: Vec<String>,
    pub(crate) video_filters: Vec<String>,
    pub(crate) pre_input_args: Vec<String>,
    pub(crate) args: Vec<String>,
    pub(crate) ext: String,
}

#[derive(PartialEq, Default)]
pub(crate) enum HWAccel {
    #[default]
    None,
    Nvenc,
    Qsv,
}

impl CommandBuilder {
    pub(crate) fn build_pre_input_args(&self) -> &Vec<String> {
        &self.pre_input_args
    }

    pub(crate) fn build_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.extend(self.args.iter().cloned());
        if !self.discard_audio && !self.audio_filters.is_empty() {
            args.push("-af".to_owned());
            args.push(self.audio_filters.join(","));
        }
        if !self.video_filters.is_empty() {
            args.push("-vf".to_owned());
            args.push(self.video_filters.join(","));
        }
        args
    }
}

impl FFmpegParameterVisitor for CommandBuilder {
    fn visit_trim(&mut self, data: &ParameterData) {
        Trim::build_command(self, data);
    }

    fn visit_disable_audio(&mut self, data: &ParameterData) {
        DisableAudio::build_command(self, data);
    }

    fn visit_audio_bitrate(&mut self, data: &ParameterData) {
        AudioBitrate::build_command(self, data);
    }

    fn visit_audio_crystalizer(&mut self, data: &ParameterData) {
        AudioCrystalizer::build_command(self, data);
    }

    fn visit_audio_volume(&mut self, data: &ParameterData) {
        AudioVolume::build_command(self, data);
    }

    fn visit_audio_pitch(&mut self, data: &ParameterData) {
        AudioPitch::build_command(self, data);
    }

    fn visit_speed_factor(&mut self, data: &ParameterData) {
        SpeedFactor::build_command(self, data);
    }

    fn visit_video_bitrate(&mut self, data: &ParameterData) {
        VideoBitrate::build_command(self, data);
    }

    fn visit_video_frame_rate(&mut self, data: &ParameterData) {
        VideoFrameRate::build_command(self, data);
    }

    fn visit_video_scale(&mut self, data: &ParameterData) {
        VideoScale::build_command(self, data);
    }

    fn visit_hardware_acceleration(&mut self, data: &ParameterData) {
        HardwareAcceleration::build_command(self, data);
    }

    fn visit_result_extension(&mut self, data: &ParameterData) {
        OutputFormat::build_command(self, data);
    }
}
