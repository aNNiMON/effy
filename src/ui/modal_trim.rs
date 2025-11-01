use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::layout::{Alignment, Margin};
use ratatui::text::Span;
use ratatui::{layout::Layout, prelude::Frame};
use ratatui::{
    layout::{Constraint, Flex, Position, Rect},
    style::{Color, Style, Stylize as _},
    symbols,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler as _;

use crate::model::TrimData;
use crate::ui::{KeyboardHandler, ModalResult, UiModal, checkbox_line, input_value_and_pos};

pub(crate) struct TrimModal {
    active_input: usize,
    ss: Input,
    to: Input,
    precise: bool,
    use_to: bool, // -t or -to
}

impl UiModal for TrimModal {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let [modal_area] = Layout::vertical([Constraint::Length(8)])
            .horizontal_margin(area.width / 5)
            .flex(Flex::Center)
            .areas(area);
        let [inputs_area, chackbox_area, hints_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(1),
        ])
        .flex(Flex::SpaceBetween)
        .areas(modal_area.inner(Margin::new(2, 1)));

        let [ss_area, to_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(inputs_area);
        let [precise_area, use_to_area] =
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(chackbox_area);

        let (ss_value, ss_x) = input_value_and_pos(&self.ss, ss_area.width);
        let (to_value, to_x) = input_value_and_pos(&self.to, to_area.width);

        let active_border_style = Style::new().blue();
        let inactive_border_style = Style::new().gray();

        Clear.render(modal_area, frame.buffer_mut());
        Block::bordered()
            .border_set(symbols::border::THICK)
            .title("Trim")
            .fg(Color::Blue)
            .render(modal_area, frame.buffer_mut());

        let mut borders = [inactive_border_style; 4];
        borders[self.active_input] = active_border_style;
        // Inputs
        Paragraph::new(ss_value)
            .block(Block::bordered().style(borders[0]).title("Start".blue()))
            .render(ss_area, frame.buffer_mut());
        let to_title = if self.use_to { "To" } else { "Duration" };
        Paragraph::new(to_value)
            .block(Block::bordered().style(borders[1]).title(to_title.blue()))
            .render(to_area, frame.buffer_mut());
        if self.active_input <= 1 {
            let (x, y) = if self.active_input == 0 {
                (ss_area.x + ss_x, ss_area.y + 1)
            } else {
                (to_area.x + to_x, to_area.y + 1)
            };
            frame.set_cursor_position(Position { x, y });
        }
        // Checkboxes
        let precise_line = checkbox_line(self.precise, "Precise", self.active_input == 2);
        Paragraph::new(precise_line)
            .alignment(Alignment::Center)
            .render(precise_area, frame.buffer_mut());
        let use_to_line = checkbox_line(self.use_to, "Use Duration/To", self.active_input == 3);
        Paragraph::new(use_to_line)
            .alignment(Alignment::Center)
            .render(use_to_area, frame.buffer_mut());
        self.render_input_hints(hints_area, frame);
    }
}

impl KeyboardHandler for TrimModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        match key.code {
            KeyCode::Esc => return ModalResult::Close,
            KeyCode::BackTab => self.active_input = (self.active_input + 3) % 4,
            KeyCode::Tab => self.active_input = (self.active_input + 1) % 4,
            KeyCode::Char(x) => match (self.active_input, x) {
                (0, '0'..='9' | '.' | ':') if self.ss.value().len() < 8 => {
                    self.ss.handle_event(&Event::Key(key));
                }
                (1, '0'..='9' | '.' | ':') if self.to.value().len() < 8 => {
                    self.to.handle_event(&Event::Key(key));
                }
                (2, ' ') => self.precise = !self.precise,
                (3, ' ') => self.use_to = !self.use_to,
                _ => {}
            },
            KeyCode::Backspace | KeyCode::Delete => match self.active_input {
                0 => {
                    self.ss.handle_event(&Event::Key(key));
                }
                1 => {
                    self.to.handle_event(&Event::Key(key));
                }
                _ => {}
            },
            KeyCode::Enter => return ModalResult::Trim,
            _ => {}
        }
        ModalResult::None
    }
}

impl From<TrimData> for TrimModal {
    fn from(data: TrimData) -> Self {
        Self {
            active_input: 0,
            ss: Input::new(data.ss.unwrap_or_default()),
            to: Input::new(data.to.unwrap_or_default()),
            precise: data.precise,
            use_to: data.use_to,
        }
    }
}

impl From<&TrimModal> for TrimData {
    fn from(model: &TrimModal) -> TrimData {
        TrimData {
            ss: Some(model.ss.value().trim().to_owned()).filter(|x| !x.is_empty()),
            to: Some(model.to.value().trim().to_owned()).filter(|x| !x.is_empty()),
            precise: model.precise,
            use_to: model.use_to,
        }
    }
}

impl TrimModal {
    fn render_input_hints(&self, area: Rect, frame: &mut Frame) {
        let keystyle = Style::default().gray().bold();
        let mut parts = vec![
            Span::styled("Enter", keystyle),
            Span::raw(": confirm  "),
            Span::styled("Esc", keystyle),
            Span::raw(": close"),
        ];
        if self.active_input > 1 {
            parts.append(&mut vec![
                Span::styled("  Space", keystyle),
                Span::raw(": toggle"),
            ]);
        }
        let parts = Line::from(parts);
        Paragraph::new(parts).render(area, frame.buffer_mut());
    }
}
