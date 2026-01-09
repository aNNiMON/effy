use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{
        Parameter, ParameterData, PresetParameter, SelectOption,
        macros::select_non_default_custom_value,
    },
    visitors::CommandBuilder,
};
pub(crate) struct VideoFrameRate;

impl VideoFrameRate {
    pub(crate) const ID: &'static str = "frate";
    pub(crate) const NAME: &'static str = "Video Frame Rate";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 10] = ["5", "10", "15", "20", "24", "25", "0", "30", "50", "60"];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::CustomSelect {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 6,
                value: Self::DEFAULT.to_owned(),
                constraints: InputConstraints {
                    length: 3,
                    input_type: InputType::PositiveInteger,
                },
                validator: Arc::new(Self::validate),
                formatter: Some(Arc::new(Self::format_value)),
            },
        )
    }

    fn validate(value: &str) -> Result<String, &str> {
        if let Ok(num) = value.parse::<i32>()
            && (0..=960).contains(&num)
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range 1..960, or 0 - original")
        }
    }

    fn format_value(value: &str) -> String {
        if value == Self::DEFAULT {
            "original".to_owned()
        } else {
            value.to_owned()
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(value) = select_non_default_custom_value!(data) {
            cb.args.push("-r".into());
            cb.args.push(value.into());
        }
    }
}

impl PresetParameter for VideoFrameRate {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        if Self::validate(preset_value).is_ok() {
            Self::set_parameter_value(data, preset_value);
        }
    }

    fn save_preset(_data: &mut ParameterData) -> String {
        todo!()
    }
}
