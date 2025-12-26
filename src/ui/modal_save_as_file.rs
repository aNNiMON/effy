use std::path::PathBuf;

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

use crate::ui::{KeyboardHandler, ModalResult, UiModal, input_value_and_pos, is_portrait};

pub(crate) struct SaveAsFileModal {
    filename: Input,
    folder: Box<str>,
    ext: Box<str>,
}

impl UiModal for SaveAsFileModal {
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

        let (display_value, x) = input_value_and_pos(&self.filename, input_area.width);

        Clear.render(modal_area, frame.buffer_mut());
        Block::bordered()
            .title("Render as".light_blue())
            .border_set(symbols::border::THICK)
            .border_style(Color::Blue)
            .render(modal_area, frame.buffer_mut());
        Paragraph::new(display_value)
            .style(Color::White)
            .block(Block::bordered().light_blue())
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
        let keystyle = Style::default().green();
        let parts = Line::from(vec![
            Span::styled("Enter", keystyle),
            Span::raw(": confirm  "),
            Span::styled("Esc", keystyle),
            Span::raw(": close"),
        ]);
        Paragraph::new(parts).render(area, frame.buffer_mut());
    }

    fn is_file_exists(&self, filename: &str) -> bool {
        let mut path = PathBuf::new().join(&*self.folder).join(filename);
        path.set_extension(&*self.ext);
        path.exists()
    }
}
