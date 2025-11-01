use core::any::Any;

pub(crate) enum ModalResult {
    None,
    Close,
    Filename(String),
    Trim,
}

pub(crate) trait UiModal: Any + KeyboardHandler {
    fn render(&self, frame: &mut ratatui::prelude::Frame);
}

pub(crate) trait KeyboardHandler {
    fn handle_key(&mut self, key: crossterm::event::KeyEvent) -> ModalResult;
}

impl dyn UiModal {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }
}
