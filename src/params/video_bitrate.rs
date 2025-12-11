use std::sync::Arc;

use crate::{
    model::{Bitrate, BitrateType, InputConstraints, InputType},
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_custom_value},
    visitors::CommandBuilder,
};

pub(crate) struct VideoBitrate;

impl VideoBitrate {
    pub(crate) const ID: &'static str = "vbitrate";
    pub(crate) const NAME: &'static str = "Video Bitrate";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 12] = [
        "16k", "32k", "0", "64k", "128k", "256k", "512k", "1M", "2M", "4M", "8M", "16M",
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::CustomSelect {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 2,
                value: Self::DEFAULT.to_owned(),
                constraints: InputConstraints {
                    length: 5,
                    input_type: InputType::Bitrate,
                },
                validator: Arc::new(Self::validate),
                formatter: Some(Arc::new(Self::format_value)),
            },
        )
    }

    fn validate(value: &str) -> Result<String, &str> {
        if let Ok(bitrate) = value.parse::<Bitrate>()
            && let Bitrate(value, unit) = &bitrate
            && (*value == 0
                || ((4..=9999).contains(value) && *unit == BitrateType::K)
                || ((1..=999).contains(value) && *unit == BitrateType::M))
        {
            Ok(bitrate.to_string())
        } else {
            Err("Invalid value. Expected range is 4k..999M, or 0 - auto")
        }
    }

    fn format_value(value: &str) -> String {
        if value == Self::DEFAULT {
            "auto".to_owned()
        } else {
            value.to_owned()
        }
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(value) = select_non_default_custom_value!(data) {
            cb.args.push("-b:v".into());
            cb.args.push(value.into());
        }
    }
}
