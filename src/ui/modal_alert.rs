use crossterm::event::KeyEvent;
use ratatui::prelude::Frame;
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph, Widget as _};
use ratatui::{layout::Constraint, style::Style};

use crate::ui::widget::BgClear;
use crate::ui::{KeyboardHandler, ModalResult, Theme, UiModal};

#[derive(Debug)]
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

#[derive(Debug)]
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
    fn render(&self, frame: &mut Frame, theme: &Theme) {
        let modal_area = frame.area().centered(
            Constraint::Length(self.width + 10),
            Constraint::Length(self.height + 3),
        );

        frame.render_widget(BgClear::new(theme.background_color()), modal_area);
        Paragraph::new(self.message)
            .style(self.modal_style(theme).bold())
            .block(
                Block::default()
                    .title_top(Line::from(self.kind.name()).centered())
                    .borders(Borders::TOP)
                    .border_type(BorderType::Double)
                    .border_style(self.border_top_style(theme))
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

    fn modal_style(&self, theme: &Theme) -> Style {
        match self.kind {
            AlertKind::Info => Style::new().fg(theme.info_text_color()),
            AlertKind::Warning => Style::new().fg(theme.warning_text_color()),
            AlertKind::Error => theme.error_bg_style(),
        }
    }

    fn border_top_style(&self, theme: &Theme) -> Style {
        match self.kind {
            AlertKind::Info => theme.info_style(),
            AlertKind::Warning => theme.warning_style(),
            AlertKind::Error => theme.error_bg_style(),
        }
    }
}
