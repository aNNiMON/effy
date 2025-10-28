use tui_input::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pane {
    Info,
    Params,
    Output,
}

#[derive(Debug, Clone)]
pub(crate) enum Modal {
    SaveFileAs(Input),
    Trim(TrimView),
}

#[derive(Debug, Clone, Default)]
pub(crate) struct TrimData {
    pub(crate) ss: Option<String>,
    pub(crate) to: Option<String>,
    pub(crate) use_to: bool,
    pub(crate) precise: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct TrimView {
    pub(crate) ss: Input,
    pub(crate) to: Input,
    pub(crate) active_input: usize,
    pub(crate) use_to: bool, // -t or -to
    pub(crate) precise: bool,
}

pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
    SaveCompleted(bool),
    Redraw,
    OpenTrimModal(TrimData),
}

impl TrimView {
    pub fn from_data(data: TrimData) -> Self {
        Self {
            ss: Input::new(data.ss.unwrap_or("".to_string())),
            to: Input::new(data.to.unwrap_or("".to_string())),
            active_input: 0,
            use_to: data.use_to,
            precise: data.precise,
        }
    }
}
