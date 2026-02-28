use tracing::debug;

use crate::{
    model::TrimData,
    params::{Parameter, ParameterData},
    visitors::CommandBuilder,
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
        if let ParameterData::Trim(trim_data) = data
            && !trim_data.is_empty()
        {
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
