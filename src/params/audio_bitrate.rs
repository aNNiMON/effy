use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct AudioBitrate;

impl AudioBitrate {
    pub(crate) const NAME: &'static str = "Audio Bitrate";
    const DEFAULT: &'static str = "auto";
    const VARIANTS: [&str; 10] = [
        "4k", "16k", "32k", "auto", "64k", "128k", "192k", "256k", "320k", "512k",
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 3,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if !cb.discard_audio
            && let Some(option) = select_non_default_option!(data)
        {
            cb.args.push("-b:a".into());
            cb.args.push(option.value.clone());
        }
    }
}
