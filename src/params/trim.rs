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
                args.push("-ss".into());
                if let Some(tmult) = time_multiplier {
                    args.push(Self::adjust_time(ss, tmult));
                } else {
                    args.push(ss.clone());
                }
            }
            if let Some(to) = &trim_data.to {
                if trim_data.use_to {
                    args.push("-to".into());
                } else {
                    args.push("-t".into());
                }
                if let Some(tmult) = time_multiplier {
                    args.push(Self::adjust_time(to, tmult));
                } else {
                    args.push(to.clone());
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

    fn adjust_time(time_str: &str, multiplier: f64) -> String {
        // Parse HH:MM:SS.mmm / MM:SS.mmm / SS.mmm
        let parts: Vec<&str> = time_str.split(':').collect();
        let seconds = match parts.len() {
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
        };
        format!("{:.3}", seconds / multiplier)
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
