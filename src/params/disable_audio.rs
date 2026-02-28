use tracing::debug;

use crate::{
    params::{Parameter, ParameterData, PresetParameter},
    visitors::CommandBuilder,
};

pub(crate) struct DisableAudio;

impl DisableAudio {
    pub(crate) const ID: &'static str = "noaudio";
    pub(crate) const NAME: &'static str = "Disable Audio";

    pub fn new_parameter() -> Parameter {
        Parameter::new(Self::ID, Self::NAME, ParameterData::Toggle { value: false }).with_order(100)
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Toggle { value } = data
            && *value
        {
            debug!("build_command disable audio");
            cb.discard_audio = true;
            cb.args.push("-an".into());
        }
    }
}

impl PresetParameter for DisableAudio {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        Self::set_parameter_value(data, preset_value);
    }

    fn save_preset(data: &ParameterData) -> Option<&str> {
        if let ParameterData::Toggle { value } = data
            && *value
        {
            Some("1")
        } else {
            None
        }
    }
}
