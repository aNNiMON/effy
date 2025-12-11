use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_custom_value},
    visitors::{CommandBuilder, HWAccel},
};

pub(crate) struct VideoScale;

impl VideoScale {
    pub(crate) const ID: &'static str = "scale";
    pub(crate) const NAME: &'static str = "Video Scale";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 7] = ["144", "240", "360", "0", "480", "720", "1080"];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::CustomSelect {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 3,
                value: Self::DEFAULT.to_owned(),
                constraints: InputConstraints {
                    length: 4,
                    input_type: InputType::PositiveInteger,
                },
                validator: Arc::new(Self::validate),
                formatter: Some(Arc::new(Self::format_value)),
            },
        )
    }

    fn validate(value: &str) -> Result<String, &str> {
        if let Ok(num) = value.parse::<i32>()
            && (num == 0 || (64..=8192).contains(&num))
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range 64..8192, or 0 - original")
        }
    }

    fn format_value(value: &str) -> String {
        if value == Self::DEFAULT {
            "original".to_owned()
        } else {
            format!("{}p", &value)
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(value) = select_non_default_custom_value!(data) {
            // Use nvenc cuda scale only if there is no other video filter
            if (cb.hwaccel == HWAccel::Nvenc) && (cb.video_filters.is_empty()) {
                cb.video_filters.push(format!("scale_cuda=-2:{}", &value));
            } else {
                cb.video_filters.push(format!("scale=-2:{}", &value));
            }
        }
    }
}
