use std::path::PathBuf;

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

use crate::ui::{KeyboardHandler, ModalResult, UiModal, input_value_and_pos};

pub(crate) struct SaveAsFileModal {
    filename: Input,
    folder: Box<str>,
    ext: Box<str>,
}

impl UiModal for SaveAsFileModal {
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

        let (display_value, x) = input_value_and_pos(&self.filename, input_area.width);

        Clear.render(modal_area, frame.buffer_mut());
        Block::bordered()
            .border_set(symbols::border::THICK)
            .title("Render as")
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

impl KeyboardHandler for SaveAsFileModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        if key.code == KeyCode::Esc {
            ModalResult::Close
        } else if key.code == KeyCode::Enter {
            let filename = self.filename.value().trim();
            let valid = !filename.is_empty() && !self.is_file_exists(filename);
            if valid {
                ModalResult::Filename(filename.to_owned())
            } else {
                ModalResult::None
            }
        } else {
            self.filename.handle_event(&Event::Key(key));
            ModalResult::None
        }
    }
}

impl SaveAsFileModal {
    pub(crate) fn new(folder: &str, filename: &str, ext: &str) -> Self {
        Self {
            filename: Input::new(filename.to_owned()),
            folder: folder.into(),
            ext: ext.into(),
        }
    }

    fn render_input_hints(area: Rect, frame: &mut Frame) {
        let parts = Line::from(vec![
            "Enter".gray().bold(),
            ": confirm  ".gray(),
            "Esc".gray().bold(),
            ": close".gray(),
        ]);
        Paragraph::new(parts).render(area, frame.buffer_mut());
    }

    fn is_file_exists(&self, filename: &str) -> bool {
        let mut path = PathBuf::new();
        path.push(&*self.folder);
        path.push(filename);
        path.set_extension(&*self.ext);
        path.exists()
    }
}
