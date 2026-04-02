use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Frame;
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, ListState};
use ratatui::{layout::Constraint, style::Stylize as _, symbols, widgets::Block};

use crate::ui::Theme;
use crate::ui::modal::{KeyboardHandler, ModalResult, UiModal};
use crate::ui::widget::BgClear;

#[derive(Debug)]
pub(crate) struct CopyModal<'a> {
    items: Vec<Line<'a>>,
    values: &'a [ModalResult],
    list_state: ListState,
}

impl<'a> UiModal for CopyModal<'a>
where
    'a: 'static,
{
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let area = frame.area();
        let modal_area = area.centered(Constraint::Length(19), Constraint::Max(6));

        frame.render_widget(BgClear::new(theme.background_color()), modal_area);
        let block = Block::bordered()
            .title("Copy to clipboard".fg(theme.modal_title_color()))
            .border_set(symbols::border::THICK)
            .border_style(theme.border_modal_style());

        frame.render_stateful_widget(
            List::new(self.items.clone())
                .block(block)
                .style(theme.text_color())
                .highlight_style(theme.list_highlight_style())
                .highlight_symbol("> "),
            modal_area,
            &mut self.list_state,
        )
    }
}

impl<'a> KeyboardHandler for CopyModal<'a> {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => ModalResult::Close,
            KeyCode::Up | KeyCode::Char('k') => {
                if let Some(selected) = self.list_state.selected()
                    && selected == 0
                {
                    self.list_state.select(Some(self.items.len() - 1));
                } else {
                    self.list_state.select_previous();
                }
                ModalResult::None
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if let Some(selected) = self.list_state.selected()
                    && selected == self.items.len() - 1
                {
                    self.list_state.select(Some(0));
                } else {
                    self.list_state.select_next();
                }
                ModalResult::None
            }
            KeyCode::Enter => {
                if let Some(selected) = self.list_state.selected() {
                    self.values[selected].clone()
                } else {
                    ModalResult::None
                }
            }
            KeyCode::Char('y') => ModalResult::CopyCommand,
            KeyCode::Char('p') => ModalResult::CopyPreset,
            KeyCode::Char('i') => ModalResult::CopyInfo,
            KeyCode::Char('o') => ModalResult::CopyOutput,
            _ => ModalResult::None,
        }
    }
}

impl<'a> CopyModal<'a> {
    pub(crate) fn new(theme: &Theme) -> Self {
        let mut list_state = ListState::default();
        list_state.select_first();
        let key_style = theme.key_style();
        Self {
            items: vec![
                Line::from(vec![
                    Span::raw("Copy command "),
                    Span::styled("y", key_style),
                ]),
                Line::from(vec![
                    Span::raw("Copy preset "),
                    Span::styled("p", key_style),
                ]),
                Line::from(vec![Span::raw("Copy info "), Span::styled("i", key_style)]),
                Line::from(vec![
                    Span::raw("Copy output "),
                    Span::styled("o", key_style),
                ]),
            ],
            values: &[
                ModalResult::CopyCommand,
                ModalResult::CopyPreset,
                ModalResult::CopyInfo,
                ModalResult::CopyOutput,
            ],
            list_state,
        }
    }
}
