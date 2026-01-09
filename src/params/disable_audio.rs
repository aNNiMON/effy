use crate::{
    params::{Parameter, ParameterData, PresetParameter},
    visitors::CommandBuilder,
};

pub(crate) struct DisableAudio;

impl DisableAudio {
    pub(crate) const ID: &'static str = "noaudio";
    pub(crate) const NAME: &'static str = "Disable Audio";

    pub fn new_parameter() -> Parameter {
        Parameter::new(Self::ID, Self::NAME, ParameterData::Toggle { value: false })
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Toggle { value } = data
            && *value
        {
            cb.discard_audio = true;
            cb.args.push("-an".into());
        }
    }
}

impl PresetParameter for DisableAudio {
    fn apply_preset(data: &mut ParameterData, preset_value: &str) {
        Self::set_parameter_value(data, preset_value);
    }

    fn save_preset(_data: &mut ParameterData) -> String {
        todo!()
    }
}
