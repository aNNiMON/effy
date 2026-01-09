use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{
        Parameter, ParameterData, PresetParameter, SelectOption,
        macros::select_non_default_custom_value,
    },
    visitors::CommandBuilder,
};

pub(crate) struct AudioVolume;

impl AudioVolume {
    pub(crate) const ID: &'static str = "volume";
    pub(crate) const NAME: &'static str = "Audio Volume";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 11] = [
        "-15", "-10", "-5", "-2", "0", "2", "5", "10", "15", "30", "50",
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::CustomSelect {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 4,
                value: Self::DEFAULT.to_owned(),
                constraints: InputConstraints {
                    length: 3,
                    input_type: InputType::Integer,
                },
                validator: Arc::new(Self::validate),
                formatter: Some(Arc::new(Self::format_value)),
            },
        )
    }

    fn validate(value: &str) -> Result<String, &str> {
        if let Ok(num) = value.parse::<i32>()
            && (-50..=50).contains(&num)
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range -50..50")
        }
    }

    fn format_value(value: &str) -> String {
        if value == Self::DEFAULT {
            "original".to_owned()
        } else {
            format!("{}dB", &value)
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if !cb.discard_audio
            && let Some(value) = select_non_default_custom_value!(data)
        {
            cb.audio_filters
                .push(format!("volume={}", Self::format_value(value)));
        }
    }
}

impl PresetParameter for AudioVolume {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        if Self::validate(preset_value).is_ok() {
            Self::set_parameter_value(data, preset_value);
        }
    }

    fn save_preset(_data: &mut ParameterData) -> String {
        todo!()
    }
}
