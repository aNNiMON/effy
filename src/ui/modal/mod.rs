use std::any::Any;

use crate::ui::Theme;

#[derive(Debug, Clone)]
pub(crate) enum ModalResult {
    None,
    /// Close modal
    Close,
    /// Filename from Render as modal
    Filename(String),
    /// Trim modal
    Trim,
    /// Modal for custom parameter values
    CustomSelect(String),
    /// Copy command (y y)
    CopyCommand,
    /// Copy preset (y p)
    CopyPreset,
}

pub(crate) trait UiModal: Any + KeyboardHandler {
    fn render(&mut self, frame: &mut ratatui::prelude::Frame, theme: &Theme);
}

pub(crate) trait KeyboardHandler {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> ModalResult;
}

impl dyn UiModal {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }
}

mod alert;
mod copy;
mod custom_select;
mod help;
mod save_as_file;
mod trim;

pub(crate) use alert::{AlertKind, AlertModal};
pub(crate) use copy::CopyModal;
pub(crate) use custom_select::CustomSelectModal;
pub(crate) use help::HelpModal;
pub(crate) use save_as_file::SaveAsFileModal;
pub(crate) use trim::TrimModal;
