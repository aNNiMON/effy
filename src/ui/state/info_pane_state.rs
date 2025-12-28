use ratatui::text::Text;

#[derive(Clone)]
pub struct InfoPaneState<'a> {
    pub text: Text<'a>,
    pub current_line: u16,
}

impl<'a> InfoPaneState<'a> {
    pub fn new(text: Text<'a>) -> InfoPaneState<'a> {
        InfoPaneState {
            text,
            current_line: 0,
        }
    }

    pub fn scroll_down(&mut self) {
        let count = self.text.lines.len() as u16;
        if self.current_line < count - 1 {
            self.current_line = self.current_line.saturating_add(1);
        }
    }

    pub fn scroll_up(&mut self) {
        if self.current_line > 0 {
            self.current_line = self.current_line.saturating_sub(1);
        }
    }
}
