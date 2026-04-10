use tracing::debug;

use crate::{
    model::CropData,
    params::{Parameter, ParameterData, PresetParameter},
    visitors::{CommandBuilder, VisitorContext},
};

pub(crate) struct Crop;

impl Crop {
    pub(crate) const ID: &'static str = "crop";
    pub(crate) const NAME: &'static str = "Crop";

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Crop(CropData {
                ..Default::default()
            }),
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Crop(crop_data) = data
            && !crop_data.is_empty()
        {
            debug!(?crop_data, "build_command");
            let mut args = Vec::new();

            debug!(?args, "crop args");
            cb.args.append(&mut args);
        }
    }
}

impl<'a> PresetParameter<'a> for Crop {
    fn apply_preset(_ctx: &VisitorContext, data: &mut ParameterData, preset_value: &str) {
        if let ParameterData::Crop(_crop_data) = data {
            todo!()
        }
    }

    fn save_preset(_ctx: &VisitorContext, data: &'a ParameterData) -> Option<String> {
        if let ParameterData::Crop(crop_data) = data {
            if crop_data.is_empty() {
                None
            } else {
                Some(format!(
                    "{}:{},{}x{}",
                    crop_data.x.as_deref().unwrap_or(""),
                    crop_data.y.as_deref().unwrap_or(""),
                    crop_data.w.as_deref().unwrap_or(""),
                    crop_data.h.as_deref().unwrap_or(""),
                ))
            }
        } else {
            None
        }
    }
}
