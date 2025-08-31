use crate::params::*;

pub(crate) trait FFmpegParameterVisitor {
    fn visit_disable_audio(&mut self, param: &DisableAudio);
    fn visit_audio_bitrate(&mut self, param: &AudioBitrate);
    fn visit_audio_crystalizer(&mut self, param: &AudioCrystalizer);
    fn visit_audio_volume(&mut self, param: &AudioVolume);
    fn visit_speed_factor(&mut self, param: &SpeedFactor);
    fn visit_video_bitrate(&mut self, param: &VideoBitrate);
    fn visit_video_frame_rate(&mut self, param: &VideoFrameRate);
    fn visit_video_scale(&mut self, param: &VideoScale);
}

pub(crate) trait FFmpegParameter {
    fn accept(&self, visitor: &mut dyn FFmpegParameterVisitor);
}

mod command_builder;

pub(crate) use command_builder::*;