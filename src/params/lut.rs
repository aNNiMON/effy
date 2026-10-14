use std::path::PathBuf;

use tracing::debug;

use crate::{
    model::{LutData, LutFiles},
    params::{Parameter, ParameterData, PresetParameter},
    visitors::{CommandBuilder, VisitorContext},
};
pub(crate) struct Lut;

impl Lut {
    pub(crate) const ID: &'static str = "lut";
    pub(crate) const NAME: &'static str = "LUT";

    pub fn new_parameter(folder: &str) -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Lut(LutData {
                folder: PathBuf::from(folder),
                files: LutFiles::NotScanned,
                selected: None,
            }),
        )
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Lut(data) = data
            && let Some(path) = &data.selected
        {
            debug!(lut_path = ?path.to_string_lossy(), "build_command");
            cb.video_filters
                .push(format!("lut3d={}", path.to_string_lossy()));
            cb.pre_output_args
                .extend(["-pix_fmt".into(), "yuv420p".into()]);
        }
    }
}

impl<'a> PresetParameter<'a> for Lut {
    fn apply_preset(_ctx: &VisitorContext, _data: &mut ParameterData, _preset_value: &str) {
        todo!("apply_preset for Lut parameter is not implemented yet");
    }

    fn save_preset(_ctx: &VisitorContext, _data: &'a ParameterData) -> Option<String> {
        todo!("save_preset for Lut parameter is not implemented yet");
    }
}
