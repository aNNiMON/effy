use crate::{params::*, visitors::FFmpegParameterVisitor};

pub(crate) struct CommandBuilder {
    pub(crate) discard_audio: bool,
    pub(crate) audio_filters: Vec<String>,
    pub(crate) video_filters: Vec<String>,
    pub(crate) args: Vec<String>,
}

impl CommandBuilder {
    pub(crate) fn new() -> Self {
        CommandBuilder {
            discard_audio: false,
            audio_filters: Vec::new(),
            video_filters: Vec::new(),
            args: Vec::new(),
        }
    }

    pub(crate) fn build(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.extend(self.args.iter().map(|s| s.to_string()));
        if !self.discard_audio && !self.audio_filters.is_empty() {
            args.push("-af".into());
            args.push(self.audio_filters.join(","));
        }
        if !self.video_filters.is_empty() {
            args.push("-vf".into());
            args.push(self.video_filters.join(","));
        }
        args
    }
}

impl FFmpegParameterVisitor for CommandBuilder {
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
}
