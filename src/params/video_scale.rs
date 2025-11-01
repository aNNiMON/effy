use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::{CommandBuilder, HWAccel},
};

pub(crate) struct VideoScale;

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
                selected_index: 3,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            // Use nvenc cuda scale only if there is no other video filter
            if (cb.hwaccel == HWAccel::Nvenc) && (cb.video_filters.is_empty()) {
                cb.video_filters
                    .push(format!("scale_cuda=-2:{}", &option.value));
            } else {
                cb.video_filters.push(format!("scale=-2:{}", &option.value));
            }
        }
    }
}
