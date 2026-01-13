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
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Nvenc,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Amf,
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    Qsv,
    #[cfg(target_os = "linux")]
    Vaapi,
    #[cfg(target_os = "macos")]
    VideoToolbox,
}

impl CommandBuilder {
    pub(crate) fn build_pre_input_args(&self) -> &[String] {
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
    fn visit_trim(&mut self, data: &mut ParameterData) {
        Trim::build_command(self, data);
    }

    fn visit_disable_audio(&mut self, data: &mut ParameterData) {
        DisableAudio::build_command(self, data);
    }

    fn visit_audio_bitrate(&mut self, data: &mut ParameterData) {
        AudioBitrate::build_command(self, data);
    }

    fn visit_audio_crystalizer(&mut self, data: &mut ParameterData) {
        AudioCrystalizer::build_command(self, data);
    }

    fn visit_audio_volume(&mut self, data: &mut ParameterData) {
        AudioVolume::build_command(self, data);
    }

    fn visit_audio_pitch(&mut self, data: &mut ParameterData) {
        AudioPitch::build_command(self, data);
    }

    fn visit_speed_factor(&mut self, data: &mut ParameterData) {
        SpeedFactor::build_command(self, data);
    }

    fn visit_video_bitrate(&mut self, data: &mut ParameterData) {
        VideoBitrate::build_command(self, data);
    }

    fn visit_video_frame_rate(&mut self, data: &mut ParameterData) {
        VideoFrameRate::build_command(self, data);
    }

    fn visit_video_scale(&mut self, data: &mut ParameterData) {
        VideoScale::build_command(self, data);
    }

    fn visit_hardware_acceleration(&mut self, data: &mut ParameterData) {
        HardwareAcceleration::build_command(self, data);
    }

    fn visit_output_format(&mut self, data: &mut ParameterData) {
        OutputFormat::build_command(self, data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::info::{Info, InfoFormat};

    // ------ Audio ------

    #[test]
    fn audio_bitrate_default() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioBitrate::new_parameter();

        cb.visit_audio_bitrate(&mut p.data);

        assert!(cb.args.is_empty());
    }

    #[test]
    fn audio_bitrate() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioBitrate::new_parameter();
        set_custom_value(&mut p, "80");

        cb.visit_audio_bitrate(&mut p.data);

        assert_eq!(cb.args, vec!["-b:a", "80k"]);
    }

    #[test]
    fn audio_crystalizer_default() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioCrystalizer::new_parameter();

        cb.visit_audio_crystalizer(&mut p.data);

        assert!(cb.audio_filters.is_empty());
    }

    #[test]
    fn audio_crystalizer() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioCrystalizer::new_parameter();
        set_custom_value(&mut p, "-4");

        cb.visit_audio_crystalizer(&mut p.data);

        assert_eq!(cb.audio_filters, vec!["crystalizer=-4"]);
    }

    #[test]
    fn audio_pitch_default() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioPitch::new_parameter();

        cb.visit_audio_pitch(&mut p.data);

        assert!(cb.audio_filters.is_empty());
    }

    #[test]
    fn audio_pitch() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioPitch::new_parameter();
        set_custom_value(&mut p, "1.5");

        cb.visit_audio_pitch(&mut p.data);

        assert_eq!(
            cb.audio_filters,
            vec!["rubberband=pitchq=quality:pitch=1.5"]
        );
    }

    #[test]
    fn audio_volume_default() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioVolume::new_parameter();

        cb.visit_audio_volume(&mut p.data);

        assert!(cb.audio_filters.is_empty());
    }

    #[test]
    fn audio_volume() {
        let mut cb = CommandBuilder::default();
        let mut p = AudioVolume::new_parameter();
        set_custom_value(&mut p, "5");

        cb.visit_audio_volume(&mut p.data);

        assert_eq!(cb.audio_filters, vec!["volume=5dB"]);
    }

    #[test]
    fn disable_audio_default() {
        let mut cb = CommandBuilder::default();
        let mut p = DisableAudio::new_parameter();

        cb.visit_disable_audio(&mut p.data);

        assert!(!cb.discard_audio);
        assert!(cb.args.is_empty());
    }

    #[test]
    fn disable_audio() {
        let mut cb = CommandBuilder::default();
        let mut p = DisableAudio::new_parameter();
        toggle_next(&mut p);

        cb.visit_disable_audio(&mut p.data);

        assert!(cb.discard_audio);
        assert_eq!(cb.args, vec!["-an"]);
    }

    #[test]
    fn disable_audio_should_disable_audio_filters() {
        let mut cb = CommandBuilder::default();
        let mut p = DisableAudio::new_parameter();
        toggle_next(&mut p);
        let mut volume = AudioVolume::new_parameter();
        set_custom_value(&mut volume, "5");

        cb.visit_disable_audio(&mut p.data);
        cb.visit_audio_volume(&mut volume.data);

        assert!(cb.discard_audio);
        assert_eq!(cb.args, vec!["-an"]);
        assert!(cb.audio_filters.is_empty());
    }

    // ------ Video ------

    #[test]
    fn video_bitrate_default() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoBitrate::new_parameter();

        cb.visit_video_bitrate(&mut p.data);

        assert!(cb.args.is_empty());
    }

    #[test]
    fn video_bitrate() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoBitrate::new_parameter();
        set_custom_value(&mut p, "2M");

        cb.visit_video_bitrate(&mut p.data);

        assert_eq!(cb.args, vec!["-b:v", "2M"]);
    }

    #[test]
    fn video_frame_rate_default() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoFrameRate::new_parameter();

        cb.visit_video_frame_rate(&mut p.data);

        assert!(cb.args.is_empty());
    }

    #[test]
    fn video_frame_rate() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoFrameRate::new_parameter();
        set_custom_value(&mut p, "25");

        cb.visit_video_frame_rate(&mut p.data);

        assert_eq!(cb.args, vec!["-r", "25"]);
    }

    #[test]
    fn video_scale_default() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoScale::new_parameter();

        cb.visit_video_scale(&mut p.data);

        assert!(cb.video_filters.is_empty());
    }

    #[test]
    fn video_scale() {
        let mut cb = CommandBuilder::default();
        let mut p = VideoScale::new_parameter();
        set_custom_value(&mut p, "600");

        cb.visit_video_scale(&mut p.data);

        assert_eq!(cb.video_filters, vec!["scale=-2:600"]);
    }

    // ------ Common ------

    #[test]
    fn speed_factor_default() {
        let mut cb = CommandBuilder::default();
        let mut p = SpeedFactor::new_parameter();

        cb.visit_speed_factor(&mut p.data);

        assert!(cb.audio_filters.is_empty());
        assert!(cb.video_filters.is_empty());
        assert_eq!(cb.speed_factor, None);
    }

    #[test]
    fn speed_factor() {
        let mut cb = CommandBuilder::default();
        let mut p = SpeedFactor::new_parameter();
        set_custom_value(&mut p, "2");

        cb.visit_speed_factor(&mut p.data);

        assert_eq!(cb.audio_filters, vec!["atempo=2"]);
        assert_eq!(cb.video_filters, vec!["setpts=PTS/2"]);
        assert_eq!(cb.speed_factor, Some(2.0));
    }

    #[test]
    fn hardware_acceleration() {
        let mut cb = CommandBuilder::default();
        let mut p = HardwareAcceleration::new_parameter();
        toggle_next(&mut p);

        cb.visit_hardware_acceleration(&mut p.data);

        #[cfg(any(target_os = "windows", target_os = "linux"))]
        assert_eq!(cb.args, vec!["-c:v", "h264_nvenc"]);

        #[cfg(target_os = "macos")]
        assert_eq!(cb.args, vec!["-c:v", "h264_videotoolbox"]);
    }

    #[test]
    fn output_format() {
        let mut cb = CommandBuilder::default();
        let info = Info {
            format: InfoFormat {
                filename: "test.mp4".to_owned(),
                ..Default::default()
            },
            streams: vec![],
        };
        let mut p = OutputFormat::new_parameter(&info, "mp4");

        cb.visit_output_format(&mut p.data);

        assert_eq!(cb.ext, "mp4");
    }

    #[test]
    fn build_args_with_filters() {
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
    fn build_args_audio_disabled() {
        let cb = CommandBuilder {
            discard_audio: true,
            audio_filters: vec!["volume=5dB".to_owned()],
            video_filters: vec!["scale=-2:720".to_owned()],
            ..Default::default()
        };
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
