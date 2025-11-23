use std::sync::Arc;

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

pub(crate) type ValidationCallback = Arc<dyn Fn(&str) -> Result<String, &str> + Send + Sync>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InputType {
    Integer,
    Decimal,
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
}
