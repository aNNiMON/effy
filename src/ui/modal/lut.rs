use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Flex, Layout, Rect},
    prelude::Frame,
    style::Stylize as _,
    symbols,
    text::{Line, Span},
    widgets::{Block, List, ListState, Paragraph, Widget as _},
};

use super::{KeyboardHandler, ModalResult, UiModal};
use crate::ui::{Theme, is_portrait, widget::BgClear};

pub(crate) struct LutModal {
    names: Vec<String>,
    state: ListState,
}

impl LutModal {
    pub(crate) fn new(names: Vec<String>, selected_index: usize) -> Self {
        Self {
            names,
            state: ListState::default().with_selected(Some(selected_index)),
        }
    }
}

impl UiModal for LutModal {
    fn render(&mut self, frame: &mut Frame, theme: &Theme) {
        let area = frame.area();
        let portrait = is_portrait(area);
        let [modal_area] = Layout::vertical([Constraint::Length(14)])
            .horizontal_margin(if portrait { 1 } else { area.width / 4 })
            .flex(Flex::Center)
            .areas(area);
        let [list_area, hints_area] =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .areas(modal_area);

        frame.render_widget(BgClear::new(theme.background_color()), modal_area);
        Block::bordered()
            .title(" LUT ".fg(theme.modal_title_color()))
            .border_set(symbols::border::THICK)
            .border_style(theme.border_modal_style())
            .render(modal_area, frame.buffer_mut());

        let list = List::new(self.names.iter().map(String::as_str))
            .style(theme.text_color())
            .highlight_style(theme.list_highlight_style())
            .highlight_symbol("> ");
        frame.render_stateful_widget(list, list_area, &mut self.state);
        self.render_status(hints_area, frame, theme);
    }
}

impl KeyboardHandler for LutModal {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        let selected = self.state.selected().unwrap_or(0);
        let count = self.names.len();
        match key.code {
            KeyCode::Esc => return ModalResult::Close,
            KeyCode::Enter => return ModalResult::Lut(selected),
            KeyCode::Up | KeyCode::Left | KeyCode::Char('k') => {
                self.state.select(Some((selected + count - 1) % count));
            }
            KeyCode::Down | KeyCode::Right | KeyCode::Char('j') => {
                self.state.select(Some((selected + 1) % count));
            }
            KeyCode::Home => self.state.select(Some(0)),
            KeyCode::End => self.state.select(Some(count - 1)),
            _ => {}
        }
        ModalResult::None
    }
}

impl LutModal {
    fn render_status(&self, area: Rect, frame: &mut Frame, theme: &Theme) {
        let key_style = theme.key_style();
        let text_style = theme.text_color();
        let line = Line::from(vec![
            Span::styled("↑/↓", key_style),
            Span::styled(": select  ", text_style),
            Span::styled("Enter", key_style),
            Span::styled(": confirm  ", text_style),
            Span::styled("Esc", key_style),
            Span::styled(": close", text_style),
        ]);
        frame.render_widget(Paragraph::new(line), area);
    }
}
