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

use crate::model::CustomSelectData;
use crate::ui::{KeyboardHandler, ModalResult, UiModal, input_value_and_pos};

pub(crate) struct CustomSelectModal {
    input: Input,
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
        Self::render_input_hints(hints_area, frame);

        frame.set_cursor_position(Position {
            x: input_area.x + x,
            y: input_area.y + 1,
        });
    }
}

impl KeyboardHandler for CustomSelectModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        if key.code == KeyCode::Esc {
            ModalResult::Close
        } else if key.code == KeyCode::Enter {
            let value = self.input.value().trim();
            ModalResult::CustomSelect(value.to_owned())
        } else {
            self.input.handle_event(&Event::Key(key));
            ModalResult::None
        }
    }
}

impl From<CustomSelectData> for CustomSelectModal {
    fn from(data: CustomSelectData) -> Self {
        Self {
            input: Input::new(data.value),
        }
    }
}

impl CustomSelectModal {
    fn render_input_hints(area: Rect, frame: &mut Frame) {
        let parts = Line::from(vec![
            "Enter".gray().bold(),
            ": confirm  ".gray(),
            "Esc".gray().bold(),
            ": close".gray(),
        ]);
        Paragraph::new(parts).render(area, frame.buffer_mut());
    }
}
