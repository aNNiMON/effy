use std::sync::Arc;

use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::prelude::Frame;
use ratatui::{
    layout::{Constraint, Flex, Layout, Position, Rect},
    style::Stylize as _,
    symbols,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget as _},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler as _;

use crate::model::{CustomSelectData, InputConstraints, InputType, ValidationCallback};
use crate::ui::widget::BgClear;
use crate::ui::{KeyboardHandler, ModalResult, Theme, UiModal, input_value_and_pos, is_portrait};

pub(crate) struct CustomSelectModal {
    name: Arc<str>,
    input: Input,
    constraints: InputConstraints,
    validation: ValidationCallback,
    error: Option<String>,
}

impl UiModal for CustomSelectModal {
    fn render(&self, frame: &mut Frame, theme: &Theme) {
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

        let (display_value, x) = input_value_and_pos(&self.input, input_area.width - 1); // subtract margin.x / 2

        frame.render_widget(BgClear::new(theme.background_color()), modal_area);
        Block::bordered()
            .title(self.name.as_ref().fg(theme.modal_title_color()))
            .border_set(symbols::border::THICK)
            .border_style(theme.border_modal_style())
            .render(modal_area, frame.buffer_mut());
        Paragraph::new(display_value)
            .style(theme.text_input_color())
            .block(theme.block_input())
            .render(input_area, frame.buffer_mut());
        self.render_status(hints_area, frame, theme);

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
    fn render_status(&self, area: Rect, frame: &mut Frame, theme: &Theme) {
        let line = if let Some(error) = &self.error {
            Line::from(error.as_str().fg(theme.error_color()).bold()).centered()
        } else {
            let key_style = theme.key_style();
            let text_style = theme.text_color();
            Line::from(vec![
                Span::styled("Enter", key_style),
                Span::styled(": confirm  ", text_style),
                Span::styled("Esc", key_style),
                Span::styled(": close", text_style),
            ])
        };
        frame.render_widget(Paragraph::new(line), area);
    }
}
