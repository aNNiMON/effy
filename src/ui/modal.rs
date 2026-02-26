use core::any::Any;

use crate::ui::Theme;

#[derive(Debug)]
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
}

pub(crate) trait UiModal: Any + KeyboardHandler {
    fn render(&self, frame: &mut ratatui::prelude::Frame, theme: &Theme);
}

pub(crate) trait KeyboardHandler {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> ModalResult;
}

impl dyn UiModal {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }
}
