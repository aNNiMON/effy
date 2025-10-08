use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct VideoScale {}

impl VideoScale {
    pub(crate) const NAME: &'static str = "Video Scale";
    const DEFAULT: &'static str = "original";
    const VARIANT_PAIRS: [(&str, &str); 7] = [
        ("144p", "144"),
        ("240p", "240"),
        ("360p", "360"),
        ("original", "original"),
        ("480p", "480"),
        ("720p", "720"),
        ("1080p", "1080"),
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_pairs(&Self::VARIANT_PAIRS),
                selected_index: 4,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            cb.video_filters.push(format!("scale=-2:{}", option.value));
        }
    }
}
