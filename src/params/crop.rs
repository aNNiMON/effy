use tracing::{debug, warn};

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

            let (x, y, w, h) = crop_data.parse();
            let args = vec![format!("crop={}:{}:{}:{}", w, h, x, y)];
            debug!(?args, "crop args");
            cb.video_filters.extend(args);
        }
    }

    pub(crate) fn parse_preset(preset_value: &str) -> Option<(&str, &str, &str, &str)> {
        let s = preset_value.strip_prefix('x')?;
        let (x, s) = s.split_once('y')?;
        let (y, s) = s.split_once('w')?;
        let (w, h) = s.split_once('h')?;
        Some((x, y, w, h))
    }
}

impl<'a> PresetParameter<'a> for Crop {
    fn apply_preset(ctx: &VisitorContext, data: &mut ParameterData, preset_value: &str) {
        if let ParameterData::Crop(crop_data) = data {
            let Some((x, y, w, h)) = Self::parse_preset(preset_value) else {
                warn!("Crop preset has invalid format: {preset_value}");
                return;
            };
            let dimensions = ctx.input_dimensions.unwrap_or_default();
            if let Err(msg) = CropData::validate(x, y, w, h, dimensions) {
                warn!("Crop preset is not valid and will be skipped: {msg}");
            } else {
                let to_option = |s: &str| Some(s.to_owned()).filter(|s| !s.is_empty());
                *crop_data = CropData {
                    x: to_option(x),
                    y: to_option(y),
                    w: to_option(w),
                    h: to_option(h),
                };
            }
        }
    }

    fn save_preset(_ctx: &VisitorContext, data: &'a ParameterData) -> Option<String> {
        if let ParameterData::Crop(crop_data) = data {
            if crop_data.is_empty() {
                None
            } else {
                Some(format!(
                    "x{}y{}w{}h{}",
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

#[cfg(test)]
mod tests {
    use super::*;

    fn apply_preset(preset: &str, dimensions: Option<(u32, u32)>) -> CropData {
        let ctx = VisitorContext {
            input_dimensions: dimensions,
            ..Default::default()
        };
        let mut data = ParameterData::Crop(CropData::default());
        Crop::apply_preset(&ctx, &mut data, preset);
        match data {
            ParameterData::Crop(c) => c,
            _ => panic!("Expected Crop data"),
        }
    }

    #[test]
    fn test_apply_crop_preset() {
        let crop = apply_preset("x10y20w100h200", Some((1920, 1080)));
        assert_eq!(crop.x.as_deref(), Some("10"));
        assert_eq!(crop.y.as_deref(), Some("20"));
        assert_eq!(crop.w.as_deref(), Some("100"));
        assert_eq!(crop.h.as_deref(), Some("200"));
    }

    #[test]
    fn test_apply_crop_preset_wh() {
        let crop = apply_preset("xyw100h200", Some((1920, 1080)));
        assert_eq!(crop.x, None);
        assert_eq!(crop.y, None);
        assert_eq!(crop.w.as_deref(), Some("100"));
        assert_eq!(crop.h.as_deref(), Some("200"));
    }

    #[test]
    fn test_apply_crop_preset_xw() {
        let crop = apply_preset("x10yw100h", Some((1920, 1080)));
        assert_eq!(crop.x.as_deref(), Some("10"));
        assert_eq!(crop.y, None);
        assert_eq!(crop.w.as_deref(), Some("100"));
        assert_eq!(crop.h, None);
    }

    #[test]
    fn test_apply_crop_preset_xy() {
        let crop = apply_preset("x10y20wh", Some((1920, 1080)));
        assert_eq!(crop.x.as_deref(), Some("10"));
        assert_eq!(crop.y.as_deref(), Some("20"));
        assert_eq!(crop.w, None);
        assert_eq!(crop.h, None);
    }

    #[test]
    fn test_save_and_apply_preset_save_apply() {
        let original = CropData {
            x: Some("50".to_string()),
            y: Some("60".to_string()),
            w: Some("300".to_string()),
            h: Some("400".to_string()),
        };
        let ctx = VisitorContext {
            input_dimensions: Some((1920, 1080)),
            ..Default::default()
        };
        let data = ParameterData::Crop(original.clone());
        let saved = Crop::save_preset(&ctx, &data).expect("save_preset failed");
        assert_eq!(saved, "x50y60w300h400");

        let applied = apply_preset(&saved, Some((1920, 1080)));
        assert_eq!(applied.x, original.x);
        assert_eq!(applied.y, original.y);
        assert_eq!(applied.w, original.w);
        assert_eq!(applied.h, original.h);
    }
}
