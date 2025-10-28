pub(crate) trait FFmpegParameterVisitor {
    fn visit_trim(&mut self, data: &ParameterData);
    fn visit_disable_audio(&mut self, data: &ParameterData);
    fn visit_audio_bitrate(&mut self, data: &ParameterData);
    fn visit_audio_crystalizer(&mut self, data: &ParameterData);
    fn visit_audio_volume(&mut self, data: &ParameterData);
    fn visit_audio_pitch(&mut self, data: &ParameterData);
    fn visit_speed_factor(&mut self, data: &ParameterData);
    fn visit_video_bitrate(&mut self, data: &ParameterData);
    fn visit_video_frame_rate(&mut self, data: &ParameterData);
    fn visit_video_scale(&mut self, data: &ParameterData);
    fn visit_hardware_acceleration(&mut self, data: &ParameterData);
}

mod command_builder;

pub(crate) use command_builder::*;

use crate::params::ParameterData;
