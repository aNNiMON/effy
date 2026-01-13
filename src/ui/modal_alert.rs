use crossterm::event::KeyEvent;
use ratatui::prelude::Frame;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Padding};
use ratatui::{
    layout::Constraint,
    style::Style,
    widgets::{Clear, Paragraph, Widget as _},
};

use crate::ui::{KeyboardHandler, ModalResult, UiModal};

pub(crate) enum AlertKind {
    Info,
    #[allow(dead_code)]
    Warning,
    Error,
}

impl AlertKind {
    fn name(&self) -> &str {
        match *self {
            AlertKind::Info => " Info ",
            AlertKind::Warning => " Warning ",
            AlertKind::Error => " Error ",
        }
    }
}

pub(crate) struct AlertModal<'a> {
    kind: AlertKind,
    message: &'a str,
    width: u16,
    height: u16,
}

impl<'a> UiModal for AlertModal<'a>
where
    'a: 'static,
{
    fn render(&self, frame: &mut Frame) {
        let modal_area = frame.area().centered(
            Constraint::Length(self.width + 10),
            Constraint::Length(self.height + 3),
        );

        Clear.render(modal_area, frame.buffer_mut());
        Paragraph::new(self.message)
            .style(self.modal_style().bold())
            .block(
                Block::default()
                    .title_top(Line::from(self.kind.name()).centered())
                    .borders(Borders::TOP)
                    .border_type(BorderType::Double)
                    .border_style(self.border_top_style())
                    .padding(Padding::vertical(1)),
            )
            .centered()
            .render(modal_area, frame.buffer_mut());
    }
}

impl KeyboardHandler for AlertModal<'_> {
    fn handle_key(&mut self, _key: KeyEvent) -> ModalResult {
        ModalResult::Close
    }
}

impl<'a> AlertModal<'a> {
    pub(crate) fn new(kind: AlertKind, message: &'a str) -> AlertModal<'a> {
        Self {
            kind,
            message,
            width: message
                .lines()
                .map(|line| line.len() as u16)
                .max()
                .unwrap_or(0),
            height: message.lines().count() as u16,
        }
    }

    fn modal_style(&self) -> Style {
        match self.kind {
            AlertKind::Info | AlertKind::Warning => Style::default().white().on_black(),
            AlertKind::Error => Style::default().white().on_red(),
        }
    }

    fn border_top_style(&self) -> Style {
        match self.kind {
            AlertKind::Info => Style::default().blue(),
            AlertKind::Warning => Style::default().yellow(),
            AlertKind::Error => Style::default().white().dim(),
        }
    }
}
