use crate::{params::*, visitors::FFmpegParameterVisitor};

pub(crate) struct CommandBuilder<'a> {
    audio_filters: Vec<String>,
    video_filters: Vec<String>,
    args: Vec<&'a str>,
}

impl<'a> CommandBuilder<'a> {
    pub(crate) fn new() -> Self {
        CommandBuilder {
            audio_filters: Vec::new(),
            video_filters: Vec::new(),
            args: Vec::new(),
        }
    }

    pub(crate) fn build(&self) -> Vec<String> {
        let mut args = Vec::new();
        args.extend(self.args.iter().map(|s| s.to_string()));
        if !self.audio_filters.is_empty() {
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

impl<'a> FFmpegParameterVisitor for CommandBuilder<'a> {
    fn visit_disable_audio(&mut self, param: &DisableAudio) {
        if *param == DisableAudio::On {
            self.args.push("-an");
        }
    }

    fn visit_audio_bitrate(&mut self, param: &AudioBitrate) {
        if *param != AudioBitrate::Auto {
            self.args.push("-b:a");
            self.args.push(param.as_str());
        }
    }

    fn visit_audio_volume(&mut self, param: &AudioVolume) {
        if *param != AudioVolume::Original {
            self.audio_filters
                .push(format!("volume={}", param.as_str()));
        }
    }

    fn visit_video_bitrate(&mut self, param: &VideoBitrate) {
        if *param != VideoBitrate::Auto {
            self.args.push("-b:v");
            self.args.push(param.as_str());
        }
    }

    fn visit_video_frame_rate(&mut self, param: &VideoFrameRate) {
        if *param != VideoFrameRate::Original {
            self.args.push("-r");
            self.args.push(param.as_str());
        }
    }
}
