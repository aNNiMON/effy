use std::{
    fmt::{Display, Formatter},
    process::ChildStdin,
    str::FromStr,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Output,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TrimData {
    pub(crate) ss: Option<String>,
    pub(crate) to: Option<String>,
    pub(crate) use_to: bool,
    pub(crate) precise: bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum BitrateType {
    K,
    M,
}
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputType {
    Integer,
    PositiveInteger,
    PositiveDecimal,
    Bitrate,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct InputConstraints {
    pub(crate) length: usize,
    pub(crate) input_type: InputType,
}

#[derive(Clone)]
pub(crate) struct CustomSelectData {
    pub(crate) name: Arc<str>,
    pub(crate) value: String,
    pub(crate) constraints: InputConstraints,
    pub(crate) validator: ValidationCallback,
}

pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
    SaveCompleted(bool),
    Redraw,
    OpenTrimModal(TrimData),
    OpenCustomSelectModal(CustomSelectData),
    RenderStarted(ChildStdin),
}
