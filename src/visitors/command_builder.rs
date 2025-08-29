use crate::{params::*, visitors::FFmpegParameterVisitor};

pub(crate) struct CommandBuilder<'a> {
    args: Vec<&'a str>,
}

impl<'a> CommandBuilder<'a> {
    pub(crate) fn new() -> Self {
        CommandBuilder { args: Vec::new() }
    }

    pub(crate) fn build(&self) -> Vec<&'a str> {
        self.args.clone()
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

    fn visit_video_bitrate(&mut self, param: &VideoBitrate) {
        if *param != VideoBitrate::Auto {
            self.args.push("-b:v");
            self.args.push(param.as_str());
        }
    }
}
