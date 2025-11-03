use std::vec;

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
use regex::Regex;
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
    error: Option<String>,
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
        self.render_status(hints_area, frame);
    }
}

impl KeyboardHandler for TrimModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        self.error.take();
        match key.code {
            KeyCode::Esc => return ModalResult::Close,
            KeyCode::BackTab => self.active_input = (self.active_input + 3) % 4,
            KeyCode::Tab => self.active_input = (self.active_input + 1) % 4,
            KeyCode::Char(x) => match (self.active_input, x) {
                (0, '0'..='9' | '.' | ':') if Self::prevalidate_value(x, self.ss.value()) => {
                    self.ss.handle_event(&Event::Key(key));
                }
                (1, '0'..='9' | '.' | ':') if Self::prevalidate_value(x, self.to.value()) => {
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
            KeyCode::Enter => {
                if let Some(msg) = self.validate() {
                    self.error = Some(msg.to_owned());
                    return ModalResult::None;
                }
                return ModalResult::Trim;
            }
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
            error: None,
        }
    }
}

impl From<&TrimModal> for TrimData {
    fn from(model: &TrimModal) -> TrimData {
        TrimData {
            ss: Some(model.ss.value().to_owned()).filter(|x| !x.is_empty()),
            to: Some(model.to.value().to_owned()).filter(|x| !x.is_empty()),
            precise: model.precise,
            use_to: model.use_to,
        }
    }
}

impl TrimModal {
    const REGEXP_SECONDS: &str = r"^([0-9]+)(\.[0-9]+)?$";
    const REGEXP_HHMMSS: &str = r"^([0-9]{1,2}:)?([0-5]?[0-9]:)([0-5]?[0-9])(\.[0-9]+)?$";

    fn render_status(&self, area: Rect, frame: &mut Frame) {
        let line = if let Some(error) = &self.error {
            Line::from(error.as_str().red().bold()).centered()
        } else {
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
            Line::from(parts)
        };
        Paragraph::new(line).render(area, frame.buffer_mut());
    }

    fn prevalidate_value(x: char, value: &str) -> bool {
        // Format 00:00:00.000 or seconds
        value.len() < 12
            && !(x == '.' && value.contains('.'))
            && !(x == ':' && value.matches(':').count() >= 2)
    }

    fn validate(&self) -> Option<&str> {
        let (ss, to) = (self.ss.value(), self.to.value());
        if !ss.is_empty() && !Self::valid_value(ss) {
            return Some("Incorrect start time format");
        }
        if !to.is_empty() && !Self::valid_value(to) {
            return Some("Incorrect duration/to format");
        }
        if self.use_to && !ss.is_empty() && !to.is_empty() {
            let ss_sec = Self::to_seconds(ss);
            let to_sec = Self::to_seconds(to);
            if ss_sec >= to_sec {
                return Some("End time must be greater than start time");
            }
        }
        if !to.is_empty() && Self::to_seconds(to) <= 0.0 {
            return Some("Duration/to must be greater than zero");
        }
        None
    }

    fn valid_value(value: &str) -> bool {
        let regexs = [Self::REGEXP_SECONDS, Self::REGEXP_HHMMSS];
        regexs.iter().any(|rstr| {
            let re = Regex::new(rstr).expect("Valid regex");
            re.is_match(value)
        })
    }

    fn to_seconds(value: &str) -> f64 {
        let parts: Vec<&str> = value.split(':').collect();
        let mut total_seconds = 0.0_f64;
        for (i, part) in parts.iter().rev().enumerate() {
            if let Ok(num) = part.parse::<f64>() {
                total_seconds += num * 60_f64.powi(i as i32);
            }
        }
        total_seconds
    }
}
