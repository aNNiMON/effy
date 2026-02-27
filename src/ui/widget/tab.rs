use ratatui::{
    style::{Color, Style},
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
    pub active_bg: Color,
    pub inactive_bg: Color,
}

impl TabStyle {
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            active_style: theme.text_color().into(),
            inactive_style: theme.text_muted_color().into(),
            active_bg: theme.tab_bg_active(),
            inactive_bg: theme.background_color(),
        }
    }
}

pub fn tabs_line<'a>(tabs: &[Tab<'a>], style: TabStyle) -> Line<'a> {
    if tabs.is_empty() {
        return Line::default();
    }
    let mut spans = Vec::with_capacity(tabs.len() * 3);
    for tab in tabs {
        let (label_style, bg) = if tab.active {
            (style.active_style, style.active_bg)
        } else {
            (style.inactive_style, style.inactive_bg)
        };
        spans.push(Span::styled("", bg));
        spans.push(Span::styled(tab.label, label_style.bg(bg)));
        spans.push(Span::styled("", bg));
    }
    Line::from(spans)
}
