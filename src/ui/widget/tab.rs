use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::ui::Theme;

// Helper function for drawing rounded tab titles

#[derive(Debug, Clone, Copy)]
pub struct Tab<'a> {
    pub label: &'a str,
    pub active: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct TabStyle {
    pub active_style: Style,
    pub inactive_style: Style,
}

impl TabStyle {
    pub fn from_theme(theme: &Theme, focused: bool) -> Self {
        let active: Style = if focused {
            theme.pane_title_color().into()
        } else {
            theme.text_muted_color().into()
        };
        let inactive: Style = theme.text_muted_color().into();
        Self {
            active_style: active.bold(),
            inactive_style: inactive.not_bold(),
        }
    }
}

pub fn tabs_line<'a>(tabs: &[Tab<'a>], style: TabStyle) -> Line<'a> {
    if tabs.is_empty() {
        return Line::default();
    }
    let mut spans = Vec::with_capacity(tabs.len() * 2 - 1);
    for (i, tab) in tabs.iter().enumerate() {
        let label_style = if tab.active {
            style.active_style
        } else {
            style.inactive_style
        };
        if i != 0 {
            spans.push(Span::raw(" • "));
        }
        spans.push(Span::styled(tab.label, label_style));
    }
    Line::from(spans)
}
