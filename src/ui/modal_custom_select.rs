use crossterm::event::{Event, KeyCode, KeyEvent};
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
use crate::ui::{KeyboardHandler, ModalResult, UiModal, input_value_and_pos};

pub(crate) struct CustomSelectModal {
    input: Input,
    constraints: InputConstraints,
    validation: ValidationCallback,
    error: Option<String>,
}

impl UiModal for CustomSelectModal {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let [modal_area] = Layout::vertical([Constraint::Length(6)])
            .horizontal_margin(area.width / 5)
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
            .border_set(symbols::border::THICK)
            .title("Enter Value")
            .fg(Color::Blue)
            .render(modal_area, frame.buffer_mut());
        Paragraph::new(display_value)
            .style(Style::new().white())
            .block(Block::bordered().gray().dim())
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
                    (InputType::Integer, '0'..='9' | '-') => {
                        self.input.handle_event(&Event::Key(key));
                    }
                    (InputType::Decimal, '0'..='9' | '.' | '-') => {
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
            Line::from(vec![
                "Enter".gray().bold(),
                ": confirm  ".gray(),
                "Esc".gray().bold(),
                ": close".gray(),
            ])
        };
        Paragraph::new(line).render(area, frame.buffer_mut());
    }
}
