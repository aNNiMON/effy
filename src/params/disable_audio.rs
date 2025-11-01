use crate::{
    params::{Parameter, ParameterData},
    visitors::CommandBuilder,
};

pub(crate) struct DisableAudio;

impl DisableAudio {
    pub(crate) const NAME: &'static str = "Disable Audio";

    pub fn new_parameter() -> Parameter {
        Parameter::new(Self::NAME, ParameterData::Toggle { value: false })
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
