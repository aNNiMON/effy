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
}

pub(crate) enum AppEvent {
    Input(crossterm::event::KeyEvent),
    AddOutput(String),
    SaveCompleted(bool),
    Redraw,
}
