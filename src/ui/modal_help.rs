use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Flex, Layout};
use ratatui::prelude::Frame;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, StatefulWidget};
use ratatui::{
    layout::Constraint,
    widgets::{Clear, Widget as _},
};

use crate::ui::state::InfoPaneState;
use crate::ui::widget::InfoPane;
use crate::ui::{KeyboardHandler, ModalResult, UiModal, is_portrait};

pub(crate) struct HelpModal<'a> {
    help_state: InfoPaneState<'a>,
}

impl UiModal for HelpModal<'static> {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let portrait = is_portrait(area);
        let [modal_area] = Layout::vertical([Constraint::Length(12)])
            .horizontal_margin(if portrait { 1 } else { area.width / 4 })
            .flex(Flex::Center)
            .areas(area);

        let block = Block::default()
            .title_top(Line::from("Help"))
            .borders(Borders::all())
            .border_type(BorderType::Thick)
            .border_style(Color::Blue);

        Clear.render(modal_area, frame.buffer_mut());
        StatefulWidget::render(
            InfoPane::new(block),
            modal_area,
            frame.buffer_mut(),
            &mut self.help_state.clone(),
        );
    }
}

impl KeyboardHandler for HelpModal<'_> {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::F(1) | KeyCode::Esc => {
                return ModalResult::Close;
            }
            KeyCode::Down | KeyCode::Char('j') => self.help_state.scroll_down(),
            KeyCode::Up | KeyCode::Char('k') => self.help_state.scroll_up(),
            _ => {}
        }
        ModalResult::None
    }
}

impl<'a> HelpModal<'a> {
    pub(crate) fn new() -> HelpModal<'a> {
        Self {
            help_state: InfoPaneState::new(Self::help_lines()),
        }
    }

    fn help_lines() -> Text<'a> {
        let mut lines = Vec::new();
        lines.push(Line::from("       Key Action").blue().bold());
        lines.extend(Self::help_navigation_lines());
        lines.extend(Self::help_render_lines());
        lines.extend(Self::help_modals_lines());
        lines.extend(Self::help_clipboard_lines());
        lines.extend(Self::logo());
        Text::from(lines)
    }

    fn logo() -> Vec<Line<'a>> {
        let style = Style::default().green().on_black();
        let mut lines = vec![""];
        lines.push("░█▀▀░█▀▀░█▀▀░█░█");
        lines.push("░█▀▀░█▀▀░█▀▀░░█░");
        lines.push("░▀▀▀░▀░░░▀░░░░▀░");
        lines.push("version");
        lines.push(env!("CARGO_PKG_VERSION"));
        lines
            .iter()
            .map(|line| Line::from(Span::styled(*line, style)).centered())
            .collect()
    }

    fn help_navigation_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::from(" Navigation:").blue().bold());
        lines.extend(Self::lines(
            &["↑", "k"],
            "Scroll up in the Info, Parameter or Output pane",
        ));
        lines.extend(Self::lines(
            &["↓", "j"],
            "Scroll down in the Info, Parameter or Output pane",
        ));
        lines.extend(Self::lines(&["Tab"], "Focus next pane"));
        lines.extend(Self::lines(&["Shift+Tab"], "Focus previous pane"));
        lines.extend(Self::lines(&["i"], "Focus Info pane"));
        lines.extend(Self::lines(
            &["←", "h"],
            "Switch the previous quick option in the Parameter pane",
        ));
        lines.extend(Self::lines(
            &["→", "l"],
            "Select the next quick option in the Parameter pane",
        ));
        lines.extend(Self::lines(&["Enter"], "Open parameter options"));
        lines.extend(Self::lines(&["Esc", "q", "Ctrl+c"], "Quit the application"));
        lines.extend(Self::lines(&["o"], "Focus Output pane"));
        lines.extend(Self::lines(&["?", "F1"], "Toggle help"));
        lines
    }

    fn help_render_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(Line::from(" Render:").blue().bold());
        lines.extend(Self::lines(&["s"], "Open the 'Render As' modal"));
        lines.extend(Self::lines(&["Ctrl+s"], "Quick render"));
        lines.extend(Self::lines(
            &["Esc", "q", "Ctrl+c"],
            "Stop rendering if it's in progress",
        ));
        lines
    }

    fn help_modals_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(Line::from(" Modals:").blue().bold());
        lines.extend(Self::lines(&["Tab"], "Focus next field"));
        lines.extend(Self::lines(&["Shift+Tab"], "Focus previous field"));
        lines.extend(Self::lines(&["Space"], "Toggle a checkbox"));
        lines.extend(Self::lines(&["Esc"], "Close an active modal"));
        lines
    }

    fn help_clipboard_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(Line::from(" Clipboard:").blue().bold());
        lines.extend(Self::lines(&["p"], "Copy a preset to clipboard"));
        lines.extend(Self::lines(&["y"], "Copy a FFmpeg command to clipboard"));
        lines
    }

    fn lines(keys: &'a [&str], v: &'a str) -> Vec<Line<'a>> {
        let key_style = Style::default().green();
        let text_style = Style::default().gray();
        let repeated_style = Style::default().dark_gray();

        keys.iter()
            .enumerate()
            .map(|(i, k)| {
                Line::from(vec![
                    Span::styled(format!("{k: >10} "), key_style),
                    Span::styled(
                        v.to_owned(),
                        if i == 0 { text_style } else { repeated_style },
                    ),
                ])
            })
            .collect()
    }
}
