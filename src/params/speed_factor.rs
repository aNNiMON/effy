use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
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
            ParameterData::Select {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 4,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            if !cb.discard_audio {
                cb.audio_filters.push(format!("atempo={}", &option.value));
            }
            cb.video_filters
                .push(format!("setpts=PTS/{}", &option.value));
        }
    }
}
