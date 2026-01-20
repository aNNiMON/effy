use crate::{
    params::{
        Parameter, ParameterData, PresetParameter, SelectOption, macros::select_non_default_option,
    },
    visitors::{CommandBuilder, HWAccel},
};

pub(crate) struct HardwareAcceleration;

impl HardwareAcceleration {
    pub(crate) const ID: &'static str = "hwaccel";
    pub(crate) const NAME: &'static str = "HW Acceleration";
    const DEFAULT: &'static str = "none";

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_pairs(&[
                    ("none", "none"),
                    #[cfg(any(target_os = "windows", target_os = "linux"))]
                    ("nvidia", "nvenc"),
                    #[cfg(any(target_os = "windows", target_os = "linux"))]
                    ("intel", "qsv"),
                    #[cfg(any(target_os = "windows", target_os = "linux"))]
                    ("amd", "amf"),
                    #[cfg(target_os = "linux")]
                    ("vaapi", "vaapi"),
                    #[cfg(target_os = "macos")]
                    ("macos", "videotoolbox"),
                ]),
                selected_index: 0,
            },
        )
        .with_order(2000)
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            match option.value.as_str() {
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                "nvenc" => {
                    cb.hwaccel = HWAccel::Nvenc;
                    cb.pre_input_args.push("-hwaccel".into());
                    cb.pre_input_args.push("cuda".into());
                    cb.pre_output_args.push("-c:v".into());
                    cb.pre_output_args.push("h264_nvenc".into());
                }
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                "amf" => {
                    cb.hwaccel = HWAccel::Amf;
                    cb.pre_output_args.push("-c:v".into());
                    cb.pre_output_args.push("h264_amf".into());
                }
                #[cfg(any(target_os = "windows", target_os = "linux"))]
                "qsv" => {
                    cb.hwaccel = HWAccel::Qsv;
                    cb.pre_input_args.push("-init_hw_device".into());
                    cb.pre_input_args.push("qsv=hw".into());
                    cb.pre_input_args.push("-filter_hw_device".into());
                    cb.pre_input_args.push("hw".into());
                    // For recompress only enable full qsv processing
                    if cb.video_filters.is_empty() {
                        cb.pre_input_args.push("-hwaccel".into());
                        cb.pre_input_args.push("qsv".into());
                        cb.pre_input_args.push("-c:v".into());
                        cb.pre_input_args.push("h264_qsv".into());
                    }
                    cb.pre_input_args.push("-hwaccel_output_format".into());
                    cb.pre_input_args.push("qsv".into());
                    cb.pre_output_args.push("-c:v".into());
                    cb.pre_output_args.push("h264_qsv".into());
                }
                #[cfg(target_os = "linux")]
                "vaapi" => {
                    cb.hwaccel = HWAccel::Vaapi;
                    cb.pre_input_args.push("-vaapi_device".into());
                    let device = std::env::var("EFFY_VAAPI_DEVICE")
                        .ok()
                        .filter(|s| {
                            s.chars()
                                .all(|c| c.is_alphanumeric() || c == '/' || c == '.')
                        })
                        .unwrap_or_else(|| "/dev/dri/renderD128".into());
                    cb.pre_input_args.push(device);
                    cb.video_filters.push("format=nv12".into());
                    cb.video_filters.push("hwupload".into());
                    cb.pre_output_args.push("-c:v".into());
                    cb.pre_output_args.push("h264_vaapi".into());
                }
                #[cfg(target_os = "macos")]
                "videotoolbox" => {
                    cb.hwaccel = HWAccel::VideoToolbox;
                    cb.pre_output_args.push("-c:v".into());
                    cb.pre_output_args.push("h264_videotoolbox".into());
                }
                _ => cb.hwaccel = HWAccel::None,
            }
        } else {
            cb.hwaccel = HWAccel::None;
        }
    }
}

impl PresetParameter for HardwareAcceleration {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        Self::set_parameter_value(data, preset_value);
    }

    fn save_preset(data: &ParameterData) -> Option<&str> {
        select_non_default_option!(data).map(|option| option.value.as_str())
    }
}
