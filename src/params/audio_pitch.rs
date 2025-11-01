use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct AudioPitch;

impl AudioPitch {
    pub(crate) const NAME: &'static str = "Audio Pitch";
    const DEFAULT: &'static str = "1";
    const VARIANTS: [&str; 8] = ["0.6", "0.8", "0.9", "1", "1.15", "1.25", "1.5", "2"];

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
                .push(format!("rubberband=pitchq=quality:pitch={}", &option.value));
        }
    }
}
