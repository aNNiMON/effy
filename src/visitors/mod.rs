pub(crate) trait FFmpegParameterVisitor {
    fn visit_trim(&mut self, data: &mut ParameterData);
    fn visit_disable_audio(&mut self, data: &mut ParameterData);
    fn visit_audio_bitrate(&mut self, data: &mut ParameterData);
    fn visit_audio_crystalizer(&mut self, data: &mut ParameterData);
    fn visit_audio_volume(&mut self, data: &mut ParameterData);
    fn visit_audio_pitch(&mut self, data: &mut ParameterData);
    fn visit_speed_factor(&mut self, data: &mut ParameterData);
    fn visit_video_bitrate(&mut self, data: &mut ParameterData);
    fn visit_video_frame_rate(&mut self, data: &mut ParameterData);
    fn visit_video_scale(&mut self, data: &mut ParameterData);
    fn visit_hardware_acceleration(&mut self, data: &mut ParameterData);
    fn visit_output_format(&mut self, data: &mut ParameterData);
}

mod command_builder;
mod preset_applier;

pub(crate) use command_builder::*;
pub(crate) use preset_applier::*;

use crate::params::ParameterData;
