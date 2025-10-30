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

pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
    SaveCompleted(bool),
    Redraw,
    OpenTrimModal(TrimData),
}
