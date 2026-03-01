use std::path::PathBuf;

use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::text::Span;
use ratatui::{layout::Layout, prelude::Frame};
use ratatui::{
    layout::{Constraint, Flex, Position, Rect},
    style::Stylize as _,
    symbols,
    text::Line,
    widgets::{Block, Paragraph, Widget as _},
};
use regex::Regex;
use tracing::debug;
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler as _;

use crate::ui::widget::BgClear;
use crate::ui::{KeyboardHandler, ModalResult, Theme, UiModal, input_value_and_pos, is_portrait};

#[derive(Debug, PartialEq)]
enum ValidationResult {
    Reset,
    Ok,
    Invalid(String),
    Exists,
}

#[derive(Debug)]
pub(crate) struct SaveAsFileModal {
    filename: Input,
    original_filename: Box<str>,
    folder: Box<str>,
    ext: Box<str>,
    validation: ValidationResult,
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

        let (display_value, x) = input_value_and_pos(&self.filename, input_area.width - 2);

        frame.render_widget(BgClear::new(theme.background_color()), modal_area);
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
            let filename = self.filename.value().trim();
            // Enter pressed after the overwrite prompt
            if self.validation == ValidationResult::Exists {
                debug!(filename, "Save as. Overwrite");
                return ModalResult::Filename(filename.to_owned());
            }
            self.validation = self.validate(filename);
            match self.validation {
                ValidationResult::Ok => {
                    debug!(filename, "Save as");
                    ModalResult::Filename(filename.to_owned())
                }
                _ => ModalResult::None,
            }
        } else {
            self.validation = ValidationResult::Reset;
            self.filename.handle_event(&Event::Key(key));
            ModalResult::None
        }
    }
}

impl SaveAsFileModal {
    pub(crate) fn new(original_filename: &str, folder: &str, filename: &str, ext: &str) -> Self {
        Self {
            filename: Input::new(filename.to_owned()),
            original_filename: original_filename.into(),
            folder: folder.into(),
            ext: ext.into(),
            validation: ValidationResult::Reset,
        }
    }

    fn render_input_hints(&self, area: Rect, frame: &mut Frame, theme: &Theme) {
        let line = match &self.validation {
            ValidationResult::Exists => {
                let error_style = theme.error_style().bold();
                Line::from(vec![
                    Span::styled("File already exists. Press ", error_style),
                    Span::styled("Enter", theme.key_style()),
                    Span::styled(" again to overwrite", error_style),
                ])
                .centered()
            }
            ValidationResult::Invalid(reason) => {
                let error_style = theme.error_style().bold();
                Line::from(vec![Span::styled(reason, error_style)]).centered()
            }
            _ => {
                let key_style = theme.key_style();
                let text_style = theme.text_color();
                Line::from(vec![
                    Span::styled("Enter", key_style),
                    Span::styled(": confirm  ", text_style),
                    Span::styled("Esc", key_style),
                    Span::styled(": close", text_style),
                ])
            }
        };
        frame.render_widget(Paragraph::new(line), area);
    }

    fn validate(&self, filename: &str) -> ValidationResult {
        if filename.is_empty() {
            ValidationResult::Invalid("Filename is empty".into())
        } else if filename == self.original_filename.as_ref() {
            ValidationResult::Invalid("Filename is the same as original".into())
        } else if Self::contains_invalid_chars(filename) {
            ValidationResult::Invalid("Filename contains invalid characters".into())
        } else if filename.len() > 200 {
            ValidationResult::Invalid("Filename is too long".into())
        } else if self.is_file_exists(filename) {
            ValidationResult::Exists
        } else {
            ValidationResult::Ok
        }
    }

    fn contains_invalid_chars(filename: &str) -> bool {
        filename.starts_with('-')
            || filename.starts_with('~')
            || std::str::from_utf8(filename.as_bytes()).is_err()
            || Regex::new(r"[/\\|<>$:\x00-\x1F\x7F\x80-\x9F]+")
                .unwrap()
                .is_match(filename)
    }

    fn is_file_exists(&self, filename: &str) -> bool {
        let mut path = PathBuf::new().join(self.folder.as_ref()).join(filename);
        path.add_extension(self.ext.as_ref());
        path.exists()
    }
}
