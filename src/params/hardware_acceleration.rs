use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::{CommandBuilder, HWAccel},
};

pub(crate) struct HardwareAcceleration;

impl HardwareAcceleration {
    pub(crate) const ID: &'static str = "hwaccel";
    pub(crate) const NAME: &'static str = "HW Acceleration";
    const DEFAULT: &'static str = "none";
    const VARIANT_PAIRS: [(&str, &str); 3] =
        [("nvidia", "nvenc"), ("none", "none"), ("intel", "qsv")];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_pairs(&Self::VARIANT_PAIRS),
                selected_index: 1,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            match option.value.as_str() {
                "nvenc" => {
                    cb.hwaccel = HWAccel::Nvenc;
                    cb.pre_input_args.push("-hwaccel".into());
                    cb.pre_input_args.push("cuda".into());
                    cb.args.push("-c:v".into());
                    cb.args.push("h264_nvenc".into());
                }
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
                    cb.args.push("-c:v".into());
                    cb.args.push("h264_qsv".into());
                }
                _ => cb.hwaccel = HWAccel::None,
            }
        } else {
            cb.hwaccel = HWAccel::None;
        }
    }
}
