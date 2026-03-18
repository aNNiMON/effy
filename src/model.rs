use std::{
    fmt::{Display, Formatter},
    process::ChildStdin,
    str::FromStr,
    sync::Arc,
};

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
