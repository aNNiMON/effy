use std::sync::Arc;

use crate::{
    model::{InputConstraints, InputType},
    params::{
        Parameter, ParameterData, PresetParameter, SelectOption,
        macros::select_non_default_custom_value,
    },
    visitors::CommandBuilder,
};

pub(crate) struct SpeedFactor;

impl SpeedFactor {
    pub(crate) const ID: &'static str = "speed";
    pub(crate) const NAME: &'static str = "Speed";
    const DEFAULT: &'static str = "1";
    const VARIANTS: [&str; 13] = [
        "0.5", "0.75", "0.8", "0.9", "1", "1.25", "1.4", "1.5", "1.6", "1.8", "2", "2.5", "3",
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
            && (0.5..=100.0).contains(&num)
        {
            Ok(num.to_string())
        } else {
            Err("Invalid value. Expected a number in range 0.5..100")
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(value) = select_non_default_custom_value!(data) {
            cb.speed_factor = value.parse().ok();
            if !cb.discard_audio {
                cb.audio_filters.push(format!("atempo={}", &value));
            }
            cb.video_filters.push(format!("setpts=PTS/{}", &value));
        }
    }
}

impl PresetParameter for SpeedFactor {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        if Self::validate(preset_value).is_ok() {
            Self::set_parameter_value(data, preset_value);
        }
    }

    fn save_preset(_data: &mut ParameterData) -> String {
        todo!()
    }
}
