use crate::{
    model::TrimData,
    params::{Parameter, ParameterData},
};

pub(crate) struct Trim {}

impl Trim {
    pub(crate) const NAME: &'static str = "Trim";

    pub fn new_parameter() -> Parameter {
        Parameter::new(Self::NAME, ParameterData::Trim(TrimData::default()))
    }
}
