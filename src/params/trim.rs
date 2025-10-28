use crate::{
    model::TrimData,
    params::{Parameter, ParameterData},
    visitors::CommandBuilder,
};

pub(crate) struct Trim {}

impl Trim {
    pub(crate) const NAME: &'static str = "Trim";

    pub fn new_parameter() -> Parameter {
        Parameter::new(Self::NAME, ParameterData::Trim(TrimData::default()))
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Trim(trim_data) = data {
            let mut args = Vec::new();
            if let Some(ss) = &trim_data.ss {
                args.push("-ss".into());
                args.push(ss.to_string());
            }
            if let Some(to) = &trim_data.to {
                if trim_data.use_to {
                    args.push("-to".into());
                } else {
                    args.push("-t".into());
                }
                args.push(to.to_string());
            }
            if trim_data.precise {
                cb.args.append(&mut args);
            } else {
                cb.pre_input_args.append(&mut args);
            }
        }
    }
}
