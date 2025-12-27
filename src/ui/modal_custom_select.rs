use std::sync::Arc;

use crossterm::event::{Event, KeyCode, KeyEvent};
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

use crate::model::{CustomSelectData, InputConstraints, InputType, ValidationCallback};
use crate::ui::{KeyboardHandler, ModalResult, UiModal, input_value_and_pos, is_portrait};

pub(crate) struct CustomSelectModal {
    name: Arc<str>,
    input: Input,
    constraints: InputConstraints,
    validation: ValidationCallback,
    error: Option<String>,
}

impl UiModal for CustomSelectModal {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let portrait = is_portrait(area);
        let [modal_area] = Layout::vertical([Constraint::Length(6)])
            .horizontal_margin(if portrait { 1 } else { area.width / 5 })
            .flex(Flex::Center)
            .areas(area);
        let [input_area, hints_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(modal_area);

        let (display_value, x) = input_value_and_pos(&self.input, input_area.width);

        Clear.render(modal_area, frame.buffer_mut());
        Block::bordered()
            .title(self.name.as_ref().light_blue())
            .border_set(symbols::border::THICK)
            .border_style(Color::Blue)
            .render(modal_area, frame.buffer_mut());
        Paragraph::new(display_value)
            .style(Color::White)
            .block(Block::bordered().light_blue())
            .render(input_area, frame.buffer_mut());
        self.render_status(hints_area, frame);

        frame.set_cursor_position(Position {
            x: input_area.x + x,
            y: input_area.y + 1,
        });
    }
}

impl KeyboardHandler for CustomSelectModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        self.error.take();
        match key.code {
            KeyCode::Esc => return ModalResult::Close,
            KeyCode::Enter => {
                let value = self.input.value().trim();
                let validation = self.validation.as_ref();
                match validation(value) {
                    Ok(valid) => return ModalResult::CustomSelect(valid),
                    Err(error) => {
                        self.error = Some(error.into());
                    }
                }
            }
            KeyCode::Backspace | KeyCode::Delete => {
                self.input.handle_event(&Event::Key(key));
            }
            KeyCode::Char(x) if self.input.value().len() < self.constraints.length => {
                match (self.constraints.input_type, x) {
                    (InputType::Integer, '0'..='9' | '-')
                    | (InputType::PositiveInteger, '0'..='9')
                    | (InputType::PositiveDecimal, '0'..='9' | '.')
                    | (InputType::Bitrate, '0'..='9' | 'k' | 'K' | 'm' | 'M') => {
                        self.input.handle_event(&Event::Key(key));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        ModalResult::None
    }
}

impl From<CustomSelectData> for CustomSelectModal {
    fn from(data: CustomSelectData) -> Self {
        Self {
            name: data.name,
            input: Input::new(data.value),
            constraints: data.constraints,
            validation: data.validator,
            error: None,
        }
    }
}

impl CustomSelectModal {
    fn render_status(&self, area: Rect, frame: &mut Frame) {
        let line = if let Some(error) = &self.error {
            Line::from(error.as_str().red().bold()).centered()
        } else {
            let keystyle = Style::default().green();
            Line::from(vec![
                Span::styled("Enter", keystyle),
                Span::raw(": confirm  "),
                Span::styled("Esc", keystyle),
                Span::raw(": close"),
            ])
        };
        Paragraph::new(line).render(area, frame.buffer_mut());
    }
}
