use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct AudioVolume;

impl AudioVolume {
    pub(crate) const ID: &'static str = "volume";
    pub(crate) const NAME: &'static str = "Audio Volume";
    const DEFAULT: &'static str = "original";
    const VARIANTS: [&str; 11] = [
        "-15dB", "-10dB", "-5dB", "-2dB", "original", "2dB", "5dB", "10dB", "15dB", "30dB", "50dB",
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 4,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if !cb.discard_audio
            && let Some(option) = select_non_default_option!(data)
        {
            cb.audio_filters.push(format!("volume={}", &option.value));
        }
    }
}
