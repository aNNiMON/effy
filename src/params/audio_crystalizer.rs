use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct AudioCrystalizer;

impl AudioCrystalizer {
    pub(crate) const NAME: &'static str = "Audio Crystalizer";
    const DEFAULT: &'static str = "0";
    const VARIANTS: [&str; 7] = ["-8", "-4", "-2", "0", "2", "4", "8"];

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
            cb.audio_filters
                .push(format!("crystalizer={}", &option.value));
        }
    }
}
