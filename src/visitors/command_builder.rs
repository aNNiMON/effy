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

    fn visit_output_format(&mut self, data: &ParameterData) {
        OutputFormat::build_command(self, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{Info, InfoFormat};

    // ------ Audio ------

    #[test]
    fn test_audio_bitrate_default() {
        let mut cb = CommandBuilder::default();
        let p = AudioBitrate::new_parameter();

        cb.visit_audio_bitrate(&p.data);

        assert!(cb.args.is_empty());
    }

    #[test]
    fn test_audio_bitrate() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioBitrate::new_parameter();
        set_custom_value(&mut p, "80");

        cb.visit_audio_bitrate(&p.data);

        assert_eq!(cb.args, vec!["-b:a", "80k"]);
    }

    #[test]
    fn test_audio_crystalizer_default() {
        let mut cb = CommandBuilder::default();
        let p = AudioCrystalizer::new_parameter();

        cb.visit_audio_crystalizer(&p.data);

        assert!(cb.audio_filters.is_empty());
    }

    #[test]
    fn test_audio_crystalizer() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioCrystalizer::new_parameter();
        set_custom_value(&mut p, "-4");

        cb.visit_audio_crystalizer(&p.data);

        assert_eq!(cb.audio_filters, vec!["crystalizer=-4"]);
    }

    #[test]
    fn test_audio_pitch_default() {
        let mut cb = CommandBuilder::default();
        let p = AudioPitch::new_parameter();

        cb.visit_audio_pitch(&p.data);

        assert!(cb.audio_filters.is_empty());
    }

    #[test]
    fn test_audio_pitch() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioPitch::new_parameter();
        set_custom_value(&mut p, "1.5");

        cb.visit_audio_pitch(&p.data);

        assert_eq!(
            cb.audio_filters,
            vec!["rubberband=pitchq=quality:pitch=1.5"]
        );
    }

    #[test]
    fn test_audio_volume_default() {
        let mut cb = CommandBuilder::default();
        let p = AudioVolume::new_parameter();

        cb.visit_audio_volume(&p.data);

        assert!(cb.audio_filters.is_empty());
    }

    #[test]
    fn test_audio_volume() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioVolume::new_parameter();
        set_custom_value(&mut p, "5");

        cb.visit_audio_volume(&p.data);

        assert_eq!(cb.audio_filters, vec!["volume=5dB"]);
    }

    #[test]
    fn test_disable_audio_default() {
        let mut cb = CommandBuilder::default();
        let p = DisableAudio::new_parameter();

        cb.visit_disable_audio(&p.data);

        assert!(!cb.discard_audio);
        assert!(cb.args.is_empty());
    }

    #[test]
    fn test_disable_audio() {
        let mut cb = CommandBuilder::default();
        let mut p = DisableAudio::new_parameter();
        toggle_next(&mut p);

        cb.visit_disable_audio(&p.data);

        assert!(cb.discard_audio);
        assert_eq!(cb.args, vec!["-an"]);
    }

    #[test]
    fn test_disable_audio_should_disable_audio_filters() {
        let mut cb = CommandBuilder::default();
        let mut p = DisableAudio::new_parameter();
        toggle_next(&mut p);
        let mut volume = AudioVolume::new_parameter();
        set_custom_value(&mut volume, "5");

        cb.visit_disable_audio(&p.data);
        cb.visit_audio_volume(&p.data);

        assert!(cb.discard_audio);
        assert_eq!(cb.args, vec!["-an"]);
        assert!(cb.audio_filters.is_empty());
    }

    // ------ Video ------

    #[test]
    fn test_video_bitrate_default() {
        let mut cb = CommandBuilder::default();
        let p = VideoBitrate::new_parameter();

        cb.visit_video_bitrate(&p.data);

        assert!(cb.args.is_empty());
    }

    #[test]
    fn test_video_bitrate() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoBitrate::new_parameter();
        set_custom_value(&mut p, "2M");

        cb.visit_video_bitrate(&p.data);

        assert_eq!(cb.args, vec!["-b:v", "2M"]);
    }

    #[test]
    fn test_video_frame_rate_default() {
        let mut cb = CommandBuilder::default();
        let p = VideoFrameRate::new_parameter();

        cb.visit_video_frame_rate(&p.data);

        assert!(cb.args.is_empty());
    }

    #[test]
    fn test_video_frame_rate() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoFrameRate::new_parameter();
        set_custom_value(&mut p, "25");

        cb.visit_video_frame_rate(&p.data);

        assert_eq!(cb.args, vec!["-r", "25"]);
    }

    #[test]
    fn test_video_scale_default() {
        let mut cb = CommandBuilder::default();
        let p = VideoScale::new_parameter();

        cb.visit_video_scale(&p.data);

        assert!(cb.video_filters.is_empty());
    }

    #[test]
    fn test_video_scale() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoScale::new_parameter();
        set_custom_value(&mut p, "600");

        cb.visit_video_scale(&p.data);

        assert_eq!(cb.video_filters, vec!["scale=-2:600"]);
    }

    // ------ Common ------

    #[test]
    fn test_speed_factor_default() {
        let mut cb = CommandBuilder::default();
        let p = SpeedFactor::new_parameter();

        cb.visit_speed_factor(&p.data);

        assert!(cb.audio_filters.is_empty());
        assert!(cb.video_filters.is_empty());
        assert_eq!(cb.speed_factor, None);
    }

    #[test]
    fn test_speed_factor() {
        let mut cb = CommandBuilder::default();
        let mut p = SpeedFactor::new_parameter();
        set_custom_value(&mut p, "2");

        cb.visit_speed_factor(&p.data);

        assert_eq!(cb.audio_filters, vec!["atempo=2"]);
        assert_eq!(cb.video_filters, vec!["setpts=PTS/2"]);
        assert_eq!(cb.speed_factor, Some(2.0));
    }

    #[test]
    fn test_hardware_acceleration() {
        let mut cb = CommandBuilder::default();
        let mut p = HardwareAcceleration::new_parameter();
        toggle_next(&mut p);

        cb.visit_hardware_acceleration(&p.data);

        assert_eq!(cb.args, vec!["-c:v", "h264_qsv"]);
    }

    #[test]
    fn test_output_format() {
        let mut cb = CommandBuilder::default();
        let info = Info {
            format: InfoFormat {
                filename: "test.mp4".to_owned(),
                ..Default::default()
            },
            streams: vec![],
        };
        let p = OutputFormat::new_parameter(&info, "mp4");

        cb.visit_output_format(&p.data);

        assert_eq!(cb.ext, "mp4");
    }

    #[test]
    fn test_build_args_with_filters() {
        let mut cb = CommandBuilder::default();
        cb.args.push("-b:v".to_owned());
        cb.args.push("2M".to_owned());
        cb.audio_filters.push("volume=5dB".to_owned());
        cb.video_filters.push("scale=-2:720".to_owned());

        let result = cb.build_args();
        assert_eq!(
            result,
            vec!["-b:v", "2M", "-af", "volume=5dB", "-vf", "scale=-2:720"]
        );
    }

    #[test]
    fn test_build_args_audio_disabled() {
        let mut cb = CommandBuilder::default();
        cb.discard_audio = true;
        cb.audio_filters.push("volume=5dB".to_owned());
        cb.video_filters.push("scale=-2:720".to_owned());

        let result = cb.build_args();
        assert_eq!(result, vec!["-vf", "scale=-2:720"]);
    }

    // ------ Private ------

    fn set_custom_value(param: &mut Parameter, new_value: &str) {
        if let ParameterData::CustomSelect { value, .. } = &mut param.data {
            *value = new_value.to_owned();
        } else {
            panic!(
                "Unable to set custom value {} to parameter {}.",
                new_value, param.name
            );
        }
    }

    fn toggle_next(param: &mut Parameter) {
        if let ParameterData::Toggle { value } = &mut param.data {
            *value = !*value;
        } else if let ParameterData::Select {
            options,
            selected_index,
        } = &mut param.data
        {
            *selected_index = if *selected_index >= options.len() - 1 {
                0
            } else {
                *selected_index + 1
            };
        } else if let ParameterData::CustomSelect {
            options,
            selected_index,
            ..
        } = &mut param.data
        {
            *selected_index = if *selected_index >= options.len() - 1 {
                0
            } else {
                *selected_index + 1
            };
        }
    }
}
