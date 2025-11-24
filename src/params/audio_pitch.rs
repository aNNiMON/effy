use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_custom_value},
    visitors::CommandBuilder,
};

pub(crate) struct AudioPitch;

impl AudioPitch {
    pub(crate) const ID: &'static str = "pitch";
    pub(crate) const NAME: &'static str = "Audio Pitch";
    const DEFAULT: &'static str = "1";
    const VARIANTS: [&str; 8] = ["0.6", "0.8", "0.9", "1", "1.15", "1.25", "1.5", "2"];

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
                    input_type: InputType::PositiveDecimal,
                },
                validator: Arc::new(Self::validate),
                formatter: None,
            },
        )
    }

    fn validate(value: &str) -> Result<String, &str> {
        if let Ok(num) = value.parse::<f64>()
            && (0.01..=100.0).contains(&num)
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range 0.01..=100.0")
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if !cb.discard_audio
            && let Some(value) = select_non_default_custom_value!(data)
        {
            cb.audio_filters
                .push(format!("rubberband=pitchq=quality:pitch={}", &value));
        }
    }
}
