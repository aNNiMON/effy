use std::{
    fmt::{Display, Formatter},
    process::ChildStdin,
    str::FromStr,
    sync::Arc,
};

use regex::Regex;
use tracing::debug;

/// Main UI panes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Output,
}

/// Trim parameters
#[derive(Debug, Clone, Default)]
pub(crate) struct TrimData {
    pub(crate) ss: Option<String>,
    pub(crate) to: Option<String>,
    pub(crate) use_to: bool,
    pub(crate) precise: bool,
}

impl TrimData {
    const REGEXP_SECONDS: &str = r"^-?([0-9]+)(\.[0-9]+)?$";
    const REGEXP_HHMMSS: &str = r"^-?([0-9]{1,2}:)?([0-5]?[0-9]:)([0-5]?[0-9])(\.[0-9]+)?$";
    const REGEXP_PERCENTS: &str = r"^-?((100(\.0+)?)|([0-9]{1,2}(\.[0-9]+)?))%$";

    pub(crate) fn is_empty(&self) -> bool {
        self.ss.is_none() && self.to.is_none()
    }

    pub(crate) fn contains_percents(&self) -> bool {
        [self.ss.as_ref(), self.to.as_ref()]
            .iter()
            .any(|s| s.is_some_and(|s| s.ends_with("%")))
    }

    pub(crate) fn normalize(&self, duration: f64) -> Result<Self, &str> {
        let mut data = self.clone();
        if let Some(ss) = &self.ss
            && ss.ends_with("%")
        {
            let percent = Self::to_percent(ss).ok_or("Invalid start time value")?;
            data.ss = Some((duration * percent / 100.0).to_string());
        }
        if let Some(to) = &self.to
            && to.ends_with("%")
        {
            let percent = Self::to_percent(to).ok_or("Invalid end time value")?;
            data.to = Some((duration * percent / 100.0).to_string());
        }
        Ok(data)
    }

    fn to_percent(s: &str) -> Option<f64> {
        s.strip_suffix('%').and_then(|s| s.parse::<f64>().ok())
    }

    pub(crate) fn validate(
        ss: &str,
        to: &str,
        use_to: bool,
        duration: Option<f64>,
    ) -> Option<&'static str> {
        // Regex format check
        if !ss.is_empty() && !Self::valid_value(ss) {
            return Some("Incorrect start time format");
        }
        if !to.is_empty() && !Self::valid_value(to) {
            return Some("Incorrect duration/to format");
        }

        debug!(ss=?ss, to=?to, duration=?duration, "Trim validate");

        // Validate start time for all cases
        // Normalize negative values relative to duration
        let ss_normalized = if ss.is_empty() {
            None
        } else {
            match (Self::to_time_value(ss, duration), duration) {
                (TimeValue::Percent(_), None) => {
                    return Some("Start time cannot be calculated without the duration");
                }
                (TimeValue::Seconds(s), Some(d)) if s < 0.0 => Some(TimeValue::Seconds(d + s)),
                (v, _) => Some(v),
            }
        };

        // Validate end time the same way
        if use_to && !to.is_empty() {
            let to_normalized = match (Self::to_time_value(to, duration), duration) {
                (TimeValue::Percent(_), None) => {
                    return Some("End time cannot be calculated without the duration");
                }
                (TimeValue::Seconds(s), Some(d)) if s < 0.0 => TimeValue::Seconds(d + s),
                (v, _) => v,
            };
            debug!(ss=?ss_normalized, to=?to_normalized, "Trim validate normalized");
            if let Some(start) = ss_normalized {
                if !start.comparable_with(&to_normalized, duration.is_some()) {
                    return Some("% cannot be used if duration is not set");
                }
                if start.gte(&to_normalized) {
                    return Some("End time must be greater than start time");
                }
            }
        }
        // Validate duration
        if !use_to && !to.is_empty() {
            let dur_time = Self::to_time_value(to, duration);
            if !dur_time.is_positive() {
                return Some("Duration must be positive");
            }
        }
        None
    }

    pub(crate) fn valid_value(value: &str) -> bool {
        let regexs = [
            Self::REGEXP_SECONDS,
            Self::REGEXP_HHMMSS,
            Self::REGEXP_PERCENTS,
        ];
        regexs.iter().any(|rstr| {
            let re = Regex::new(rstr).expect("Valid regex");
            re.is_match(value)
        })
    }

    fn to_time_value(value: &str, duration: Option<f64>) -> TimeValue {
        if value.ends_with("%") {
            Self::parse_percent(value, duration)
        } else {
            let parts: Vec<&str> = value.split(':').collect();
            let mut total_seconds = 0.0_f64;
            for (i, part) in parts.iter().rev().enumerate() {
                if let Ok(num) = part.parse::<f64>() {
                    total_seconds += num * 60_f64.powi(i as i32);
                }
            }
            TimeValue::Seconds(total_seconds)
        }
    }

    fn parse_percent(value: &str, duration: Option<f64>) -> TimeValue {
        let percent = value
            .strip_suffix("%")
            .map(|s| s.parse::<f64>().unwrap_or_default())
            .unwrap_or_default();
        if let Some(duration) = duration {
            TimeValue::Seconds(duration * percent / 100.0)
        } else {
            TimeValue::Percent(percent)
        }
    }
}

impl Display for TrimData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}..{} {}",
            if self.precise { "!" } else { "~" },
            self.ss.as_deref().unwrap_or("start"),
            self.to.as_deref().unwrap_or("end"),
            if self.use_to { "(to)" } else { "(duration)" },
        )
    }
}

#[derive(Debug)]
enum TimeValue {
    Seconds(f64),
    Percent(f64),
}

impl TimeValue {
    fn comparable_with(&self, other: &Self, has_duration: bool) -> bool {
        match (self, other) {
            (TimeValue::Seconds(_), TimeValue::Seconds(_)) => true,
            (TimeValue::Percent(_), TimeValue::Percent(_)) => has_duration,
            (TimeValue::Seconds(_), TimeValue::Percent(_)) => has_duration,
            (TimeValue::Percent(_), TimeValue::Seconds(_)) => has_duration,
        }
    }

    fn gte(&self, other: &Self) -> bool {
        match (self, other) {
            (TimeValue::Seconds(a), TimeValue::Seconds(b)) => a >= b,
            (TimeValue::Percent(a), TimeValue::Percent(b)) => a >= b,
            (TimeValue::Seconds(a), TimeValue::Percent(b)) => a >= b,
            (TimeValue::Percent(a), TimeValue::Seconds(b)) => a >= b,
        }
    }

    fn is_positive(&self) -> bool {
        match self {
            TimeValue::Seconds(v) => *v > 0.0,
            TimeValue::Percent(v) => *v > 0.0,
        }
    }
}

/// Bitrate type
#[derive(Debug, PartialEq)]
pub(crate) enum BitrateType {
    K,
    M,
}

/// Bitrate value for Video parameter only, Audio uses kilobytes
#[derive(Debug)]
pub(crate) struct Bitrate(pub u32, pub BitrateType);

impl Display for BitrateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if *self == BitrateType::K { "k" } else { "M" })
    }
}

impl Display for Bitrate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0 {
            write!(f, "0")
        } else {
            write!(f, "{}{}", self.0, self.1)
        }
    }
}

impl FromStr for Bitrate {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let (value_str, unit_str) = s
            .find(|c: char| !c.is_ascii_digit())
            .map_or((s, ""), |idx| s.split_at(idx));
        if value_str.is_empty() {
            Ok(Bitrate(0, BitrateType::K))
        } else {
            let value = value_str.parse::<u32>().map_err(|_| "Invalid value")?;
            let unit = match unit_str.to_ascii_lowercase().as_str() {
                "" | "k" | "K" => BitrateType::K,
                "m" | "M" => BitrateType::M,
                _ => return Err("Invalid unit"),
            };
            Ok(Bitrate(value, unit))
        }
    }
}

pub(crate) type ValidationCallback = Arc<dyn Fn(&str) -> Result<String, &str> + Send + Sync>;
pub(crate) type ValueFormatter = Arc<dyn Fn(&str) -> String + Send + Sync>;

/// Input widget types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputType {
    Integer,
    PositiveInteger,
    PositiveDecimal,
    Bitrate,
}

/// Input constraints, max length in characters and input type
#[derive(Debug, Clone, Copy)]
pub(crate) struct InputConstraints {
    pub(crate) length: usize,
    pub(crate) input_type: InputType,
}

/// User-specified data
#[derive(Clone)]
pub(crate) struct CustomSelectData {
    pub(crate) name: Arc<str>,
    pub(crate) value: String,
    pub(crate) constraints: InputConstraints,
    pub(crate) validator: ValidationCallback,
}

/// Application events
pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
    SaveCompleted(bool),
    Redraw,
    OpenTrimModal(TrimData),
    OpenCustomSelectModal(CustomSelectData),
    RenderStarted(ChildStdin),
}
