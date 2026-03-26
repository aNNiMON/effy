use tracing::{debug, warn};

use crate::{
    model::TrimData,
    params::{Parameter, ParameterData, PresetParameter},
    visitors::{CommandBuilder, VisitorContext},
};

pub(crate) struct Trim;

impl Trim {
    pub(crate) const ID: &'static str = "trim";
    pub(crate) const NAME: &'static str = "Trim";

    pub fn new_parameter() -> Parameter {
        Parameter::new(
            Self::ID,
            Self::NAME,
            ParameterData::Trim(TrimData {
                use_to: true,
                ..Default::default()
            }),
        )
        .with_order(1600)
    }

    pub fn build_command(cb: &mut CommandBuilder, data: &ParameterData) {
        if let ParameterData::Trim(trim_raw) = data
            && !trim_raw.is_empty()
        {
            let has_percents = trim_raw.contains_percents();
            if has_percents && cb.ctx.input_duration.is_none() {
                warn!("Trim contains percents and will be ignored when input duration is not set");
                return;
            }

            let trim_data = if has_percents {
                let duration = cb.ctx.input_duration.unwrap();
                debug!(?trim_raw, ?duration, "build_command before normalization");
                match trim_raw.normalize(duration) {
                    Ok(td) => td,
                    Err(err) => {
                        warn!("Failed to normalize trim: {}", err);
                        return;
                    }
                }
            } else {
                trim_raw.clone()
            };

            debug!(?trim_data, "build_command");
            let mut args = Vec::new();

            let time_multiplier = cb
                .speed_factor
                .filter(|spd| trim_data.precise && (*spd - 1.0).abs() > 0.00001);

            if let Some(ss) = &trim_data.ss {
                let ss_normalized = if let Some(ss_abs) = ss.strip_prefix("-") {
                    if let Some(dur) = cb.ctx.input_duration {
                        args.push("-ss".into());
                        format!("{:.5}", dur - Self::to_seconds(ss_abs))
                    } else if !trim_data.precise {
                        // -sseof works pretty bad if paired with -to
                        // and also doesn't work in precise mode.
                        // Since duration is unknown, there are no other options left.
                        args.push("-sseof".into());
                        ss.to_owned()
                    } else {
                        warn!("Duration is unknown, -ss/-sseof {} cannot be used", ss);
                        return;
                    }
                } else {
                    args.push("-ss".into());
                    ss.to_owned()
                };

                if let Some(tmult) = time_multiplier {
                    args.push(Self::adjust_time(&ss_normalized, tmult));
                } else {
                    args.push(ss_normalized);
                }
            }
            if let Some(to) = &trim_data.to {
                let to_normalized = if trim_data.use_to
                    && let Some(to_abs) = to.strip_prefix("-")
                    && let Some(dur) = cb.ctx.input_duration
                {
                    args.push("-to".into());
                    format!("{:.5}", dur - Self::to_seconds(to_abs))
                } else if trim_data.use_to {
                    args.push("-to".into());
                    to.to_owned()
                } else {
                    args.push("-t".into());
                    to.to_owned()
                };

                if let Some(tmult) = time_multiplier {
                    args.push(Self::adjust_time(&to_normalized, tmult));
                } else {
                    args.push(to_normalized);
                }
            }
            debug!(?args, "trim args");
            if trim_data.precise {
                cb.args.append(&mut args);
            } else {
                cb.pre_input_args.append(&mut args);
            }
        }
    }

    fn to_seconds(time_str: &str) -> f64 {
        // Parse HH:MM:SS.mmm / MM:SS.mmm / SS.mmm
        let parts: Vec<&str> = time_str.split(':').collect();
        match parts.len() {
            1 => parts[0].parse::<f64>().unwrap_or_default(),
            2 => {
                let minutes = parts[0].parse::<f64>().unwrap_or_default();
                let secs = parts[1].parse::<f64>().unwrap_or_default();
                minutes * 60.0 + secs
            }
            3 => {
                let hours = parts[0].parse::<f64>().unwrap_or_default();
                let minutes = parts[1].parse::<f64>().unwrap_or_default();
                let secs = parts[2].parse::<f64>().unwrap_or_default();
                hours * 3600.0 + minutes * 60.0 + secs
            }
            _ => 0.0,
        }
    }

    fn adjust_time(time_str: &str, multiplier: f64) -> String {
        format!("{:.5}", Self::to_seconds(time_str) / multiplier)
    }
}

impl<'a> PresetParameter<'a> for Trim {
    fn apply_preset(ctx: &VisitorContext, data: &mut ParameterData, preset_value: &str) {
        if let ParameterData::Trim(trim_raw) = data {
            // !start..end+
            let mut result = TrimData::default();
            let s = preset_value.trim();
            // Precise
            let s = if let Some(updated) = s.strip_prefix("!") {
                result.precise = true;
                updated
            } else {
                s
            };
            // Use duration (+) or to
            let s = if let Some(updated) = s.strip_suffix("+") {
                result.use_to = false;
                updated
            } else {
                result.use_to = true;
                s
            };
            // start..end
            let parts: Vec<&str> = s.split("..").collect();
            if parts.len() != 2 {
                return;
            }
            let ss = parts[0];
            if TrimData::valid_value(ss) {
                result.ss = Some(ss.to_owned());
            }
            let to = parts[1];
            if TrimData::valid_value(to) {
                result.to = Some(to.to_owned());
            }
            if let Some(msg) = TrimData::validate(ss, to, result.use_to, ctx.input_duration) {
                warn!("Trim preset is not valid and will be skipped: {}", msg);
            } else {
                *trim_raw = result;
            }
        }
    }

    fn save_preset(_ctx: &VisitorContext, data: &'a ParameterData) -> Option<String> {
        if let ParameterData::Trim(trim_raw) = data {
            if trim_raw.is_empty() {
                None
            } else {
                Some(format!(
                    "{}{}..{}{}",
                    if trim_raw.precise { "!" } else { "" },
                    trim_raw.ss.as_deref().unwrap_or(""),
                    trim_raw.to.as_deref().unwrap_or(""),
                    if !trim_raw.use_to { "+" } else { "" }
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
    use crate::model::TrimData;
    use crate::params::ParameterData;
    use crate::visitors::VisitorContext;

    fn apply_preset(preset: &str) -> TrimData {
        let ctx = VisitorContext {
            input_duration: Some(100.0),
        };
        let mut data = ParameterData::Trim(TrimData::default());
        Trim::apply_preset(&ctx, &mut data, preset);
        match data {
            ParameterData::Trim(t) => t,
            _ => panic!("Expected Trim data"),
        }
    }

    fn save_preset(trim_data: TrimData) -> Option<String> {
        let ctx = VisitorContext {
            input_duration: Some(100.0),
        };
        let data = ParameterData::Trim(trim_data);
        Trim::save_preset(&ctx, &data)
    }

    #[test]
    fn test_apply_precise_preset() {
        let t = apply_preset("!10..20");
        assert_eq!(t.ss.as_deref(), Some("10"));
        assert_eq!(t.to.as_deref(), Some("20"));
        assert!(t.use_to);
        assert!(t.precise);
    }

    #[test]
    fn test_apply_precise_negative_preset() {
        let t = apply_preset("!-20..90");
        assert_eq!(t.ss.as_deref(), Some("-20"));
        assert_eq!(t.to.as_deref(), Some("90"));
        assert!(t.use_to);
        assert!(t.precise);
    }

    #[test]
    fn test_apply_both_negative_preset() {
        let t = apply_preset("!-20..-10");
        assert_eq!(t.ss.as_deref(), Some("-20"));
        assert_eq!(t.to.as_deref(), Some("-10"));
        assert!(t.use_to);
        assert!(t.precise);
    }

    #[test]
    fn test_apply_duration_preset() {
        let t = apply_preset("10..20+");
        assert_eq!(t.ss.as_deref(), Some("10"));
        assert_eq!(t.to.as_deref(), Some("20"));
        assert!(!t.use_to);
        assert!(!t.precise);
    }

    #[test]
    fn test_apply_invalid_preset() {
        let presets = [
            "start..end",
            "!90%..10", // normalized start >= end
            // sseof
            "!-10..20",  // start 90 >= end 20
            "-30%..20%", // normalized start >= normalized end
            "-10..0+",   // duration 0
            "-10..-20+", // negative duration
        ];
        for preset in presets {
            let t = apply_preset(preset);
            assert_eq!(t.ss, None, "preset {}", preset);
            assert_eq!(t.to, None, "preset {}", preset);
            assert!(!t.use_to, "preset {}", preset);
            assert!(!t.precise, "preset {}", preset);
        }
    }

    #[test]
    fn test_apply_negative_percent_preset() {
        let t = apply_preset("-30%..-20%");
        assert_eq!(t.ss.as_deref(), Some("-30%"));
        assert_eq!(t.to.as_deref(), Some("-20%"));
        assert!(t.use_to);
        assert!(!t.precise);
    }

    #[test]
    fn test_apply_missing_split_preset() {
        let t = apply_preset("10+");
        assert_eq!(t.ss, None);
        assert_eq!(t.to, None);
        assert!(!t.use_to);
    }

    #[test]
    fn test_apply_empty_start_preset() {
        let t = apply_preset("!..20");
        assert_eq!(t.ss, None);
        assert_eq!(t.to.as_deref(), Some("20"));
        assert!(t.use_to);
        assert!(t.precise);
    }

    #[test]
    fn test_apply_empty_end_preset() {
        let t = apply_preset("00:10..");
        assert_eq!(t.ss.as_deref(), Some("00:10"));
        assert_eq!(t.to, None);
        assert!(t.use_to);
    }

    #[test]
    fn test_apply_percent_preset() {
        let t = apply_preset("10%..20%");
        assert_eq!(t.ss.as_deref(), Some("10%"));
        assert_eq!(t.to.as_deref(), Some("20%"));
    }

    #[test]
    fn test_apply_hhmmss_preset() {
        let t = apply_preset("00:01:23.456..00:02:00");
        assert_eq!(t.ss.as_deref(), Some("00:01:23.456"));
        assert_eq!(t.to.as_deref(), Some("00:02:00"));
    }

    #[test]
    fn test_save_empty_preset() {
        let p = save_preset(TrimData::default());
        assert_eq!(p, None);
    }

    #[test]
    fn test_save_precise_duration_preset() {
        let p = save_preset(TrimData {
            ss: Some("10".to_string()),
            to: Some("20".to_string()),
            precise: true,
            use_to: false,
        });
        assert_eq!(p, Some("!10..20+".to_string()));
    }

    #[test]
    fn test_save_duration_only_preset() {
        let p = save_preset(TrimData {
            ss: None,
            to: Some("30".to_string()),
            precise: false,
            use_to: false,
        });
        assert_eq!(p, Some("..30+".to_string()));
    }

    #[test]
    fn test_save_start_only_preset() {
        let p = save_preset(TrimData {
            ss: Some("00:20:45".to_string()),
            to: None,
            precise: true,
            use_to: true,
        });
        assert_eq!(p, Some("!00:20:45..".to_string()));
    }

    #[test]
    fn test_save_percent_preset() {
        let p = save_preset(TrimData {
            ss: Some("10%".to_string()),
            to: Some("20%".to_string()),
            precise: false,
            use_to: true,
        });
        assert_eq!(p, Some("10%..20%".to_string()));
    }

    #[test]
    fn test_save_percent_negative_preset() {
        let p = save_preset(TrimData {
            ss: Some("10%".to_string()),
            to: Some("-20%".to_string()),
            precise: false,
            use_to: true,
        });
        assert_eq!(p, Some("10%..-20%".to_string()));
    }

    #[test]
    fn test_save_percent_mixed_preset() {
        let p = save_preset(TrimData {
            ss: Some("50.25".to_string()),
            to: Some("80%".to_string()),
            precise: true,
            use_to: true,
        });
        assert_eq!(p, Some("!50.25..80%".to_string()));
    }
}
