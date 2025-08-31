use crate::params::*;

pub(crate) trait FFmpegParameterVisitor {
    fn visit_disable_audio(&mut self, param: &DisableAudio);
    fn visit_audio_bitrate(&mut self, param: &AudioBitrate);
    fn visit_video_bitrate(&mut self, param: &VideoBitrate);
    fn visit_video_frame_rate(&mut self, param: &VideoFrameRate);
}

pub(crate) trait FFmpegParameter {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor);
}

mod command_builder;

pub(crate) use command_builder::*;