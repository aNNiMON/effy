use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{
        Parameter, ParameterData, PresetParameter, SelectOption,
        macros::select_non_default_custom_value,
    },
    visitors::CommandBuilder,
};

pub(crate) struct AudioCrystalizer;

impl AudioCrystalizer {
    pub(crate) const ID: &'static str = "crystalizer";
    pub(crate) const NAME: &'static str = "Audio Crystalizer";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 7] = ["-8", "-4", "-2", "0", "2", "4", "8"];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::CustomSelect {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 3,
                value: Self::DEFAULT.to_owned(),
                constraints: InputConstraints {
                    length: 3,
                    input_type: InputType::Integer,
                },
                validator: Arc::new(Self::validate),
                formatter: None,
            },
        )
    }

    fn validate(value: &str) -> Result<String, &str> {
        if let Ok(num) = value.parse::<i32>()
            && (-10..=10).contains(&num)
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range -10..10")
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if !cb.discard_audio
            && let Some(value) = select_non_default_custom_value!(data)
        {
            cb.audio_filters.push(format!("crystalizer={}", &value));
        }
    }
}

impl PresetParameter for AudioCrystalizer {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        if Self::validate(preset_value).is_ok() {
            Self::set_parameter_value(data, preset_value);
        }
    }

    fn save_preset(data: &ParameterData) -> Option<&str> {
        select_non_default_custom_value!(data)
    }
}
