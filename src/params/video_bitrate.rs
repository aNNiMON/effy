use crate::{
    params::{Parameter, ParameterData, SelectOption, macros::select_non_default_option},
    visitors::CommandBuilder,
};

pub(crate) struct VideoBitrate;

impl VideoBitrate {
    pub(crate) const ID: &'static str = "vbitrate";
    pub(crate) const NAME: &'static str = "Video Bitrate";
    const DEFAULT: &'static str = "auto";
    const VARIANTS: [&str; 12] = [
        "16k", "32k", "auto", "64k", "128k", "256k", "512k", "1M", "2M", "4M", "8M", "16M",
    ];

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Select {
                options: SelectOption::from_slice(&Self::VARIANTS),
                selected_index: 2,
            },
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let Some(option) = select_non_default_option!(data) {
            cb.args.push("-b:v".into());
            cb.args.push(option.value.clone());
        }
    }
}
