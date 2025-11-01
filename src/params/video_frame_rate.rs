use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct VideoFrameRate;

impl VideoFrameRate {
    pub(crate) const NAME: &'static str = "Video Frame Rate";
    const DEFAULT: &'static str = "original";
    const VARIANTS: [&str; 10] = [
        "5", "10", "15", "20", "24", "25", "original", "30", "50", "60",
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 6,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            cb.args.push("-r".into());
            cb.args.push(option.value.clone());
        }
    }
}
