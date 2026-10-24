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
            cb.video_filters.push(format!(
                "lut3d=file={}",
                Self::escape_filter_path(&path.to_string_lossy())
            ));
            cb.pre_output_args
                .extend(["-pix_fmt".into(), "yuv420p".into()]);
        }
    }

    fn escape_filter_path(path: &str) -> String {
        // Normalize Windows paths
        let normalized = path.replace('\\', "/");

        let mut out = String::with_capacity(normalized.len() + 2);
        out.push('\'');
        for ch in normalized.chars() {
            match ch {
                // Escape single quote by closing, eacaping and reopening it again
                '\'' => out.push_str(r"'\\\''"),
                // Escape filter option separator
                ':' => out.push_str("\\:"),
                _ => out.push(ch),
            }
        }
        out.push('\'');
        out
    }
}

impl<'a> PresetParameter<'a> for Lut {
    fn apply_preset(_ctx: &VisitorContext, data: &mut ParameterData, preset_value: &str) {
        if let ParameterData::Lut(data) = data {
            if preset_value == "none" {
                data.selected = None;
            } else {
                let path = PathBuf::from(preset_value);
                if LutFiles::is_lut(&path) && path.is_file() && path.exists() {
                    data.selected = Some(path);
                }
            }
        }
    }

    fn save_preset(_ctx: &VisitorContext, data: &'a ParameterData) -> Option<String> {
        if let ParameterData::Lut(data) = data {
            data.selected
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lut;

    #[test]
    fn escape_filter_path() {
        let cases = [
            ("lut.cube", "'lut.cube'"),
            ("C:\\My LUTs\\warm.cube", "'C\\:/My LUTs/warm.cube'"),
            (
                "C:\\Marvel's LUT\\v1\\warm.cube",
                "'C\\:/Marvel'\\\\\\''s LUT/v1/warm.cube'",
            ),
            (
                "/home/user/my lut: gold.cube",
                "'/home/user/my lut\\: gold.cube'",
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(Lut::escape_filter_path(input), expected, "input: {input}");
        }
    }
}
