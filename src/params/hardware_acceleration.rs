use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::{CommandBuilder, HWAccel},
};

pub(crate) struct HardwareAcceleration {}

impl HardwareAcceleration {
    pub(crate) const NAME: &'static str = "HW Acceleration";
    const DEFAULT: &'static str = "none";
    const VARIANT_PAIRS: [(&str, &str); 2] = [("nvidia", "nvenc"), ("none", "none")];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
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
                _ => cb.hwaccel = HWAccel::None,
            };
        } else {
            cb.hwaccel = HWAccel::None;
        }
    }
}
