#[derive(Debug, Clone)]
pub struct OutputPaneState {
    pub output: String,
    pub current_line: u16,
}

impl OutputPaneState {
    pub fn new(output: String) -> OutputPaneState {
        OutputPaneState {
            output,
            current_line: 0,
        }
    }

    pub fn set_output(&mut self, output: &str) {
        output.clone_into(&mut self.output);
        self.current_line = 0;
    }

    pub fn add_output(&mut self, output: &str) {
        self.output.push_str(output);
        self.current_line = 0;
    }

    pub fn scroll_down(&mut self) {
        let count = self.output.lines().count() as u16;
        if count > 0 {
            self.current_line = self.current_line.saturating_sub(1);
        }
    }

    pub fn scroll_up(&mut self) {
        let count = self.output.lines().count() as u16;
        if self.current_line < count {
            self.current_line = self.current_line.saturating_add(1).min(count);
        }
    }
}
