use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_custom_value},
    visitors::CommandBuilder,
};

pub(crate) struct AudioBitrate;

impl AudioBitrate {
    pub(crate) const ID: &'static str = "abitrate";
    pub(crate) const NAME: &'static str = "Audio Bitrate";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 10] = [
        "4", "16", "32", "0", "64", "128", "192", "256", "320", "512",
    ];

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
            && (num == 0 || (4..=1024).contains(&num))
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range 4..1024, or 0 - auto")
        }
    }

    fn format_value(value: &str) -> String {
        if value == Self::DEFAULT {
            "auto".to_owned()
        } else {
            format!("{}k", &value)
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if !cb.discard_audio
            && let Some(value) = select_non_default_custom_value!(data)
        {
            cb.args.push("-b:a".into());
            cb.args.push(Self::format_value(value));
        }
    }
}
