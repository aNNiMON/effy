use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::text::Span;
use ratatui::{layout::Layout, prelude::Frame};
use ratatui::{
    layout::{Constraint, Flex, Position, Rect},
    style::Stylize as _,
    symbols,
    text::Line,
    widgets::{Block, Clear, Paragraph, Widget as _},
};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler as _;

use crate::ui::{KeyboardHandler, ModalResult, Theme, UiModal, input_value_and_pos, is_portrait};

#[derive(Debug, PartialEq)]
enum Overwrite {
    Reset,
    Prompted,
    Confirmed,
}

#[derive(Debug)]
pub(crate) struct SaveAsFileModal {
    filename: Input,
    folder: Box<str>,
    ext: Box<str>,
    overwrite: Overwrite,
}

impl UiModal for SaveAsFileModal {
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

        let (display_value, x) = input_value_and_pos(&self.filename, input_area.width);

        frame.render_widget(Clear, modal_area);
        Block::bordered()
            .title("Render as".fg(theme.modal_title_color()))
            .border_set(symbols::border::THICK)
            .border_style(theme.border_modal_style())
            .render(modal_area, frame.buffer_mut());
        Paragraph::new(display_value)
            .style(theme.text_input_color())
            .block(theme.block_input())
            .render(input_area, frame.buffer_mut());
        self.render_input_hints(hints_area, frame, theme);

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
            if self.overwrite == Overwrite::Prompted {
                self.overwrite = Overwrite::Confirmed;
            }
            let filename = self.filename.value().trim();
            let valid = !filename.is_empty() && !self.is_file_exists(filename);
            if valid || self.overwrite == Overwrite::Confirmed {
                ModalResult::Filename(filename.to_owned())
            } else {
                self.overwrite = Overwrite::Prompted;
                ModalResult::None
            }
        } else {
            self.overwrite = Overwrite::Reset;
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
            overwrite: Overwrite::Reset,
        }
    }

    fn render_input_hints(&self, area: Rect, frame: &mut Frame, theme: &Theme) {
        let line = if self.overwrite == Overwrite::Prompted {
            let error = theme.error_style().bold();
            Line::from(vec![
                Span::styled("File already exists. Press ", error),
                Span::styled("Enter", theme.key_style()),
                Span::styled(" again to overwrite", error),
            ])
            .centered()
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

    fn is_file_exists(&self, filename: &str) -> bool {
        let mut path = PathBuf::new().join(&*self.folder).join(filename);
        path.set_extension(&*self.ext);
        path.exists()
    }
}
