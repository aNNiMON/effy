use ratatui::{
    prelude::Stylize as _,
    style::{Color, Style},
    text::{Line, Span},
};

#[derive(Clone, Copy)]
pub struct Tab<'a> {
    pub label: &'a str,
    pub active: bool,
}

#[derive(Clone, Copy)]
pub struct TabStyle {
    pub active_style: Style,
    pub inactive_style: Style,
    pub active_bg: Color,
    pub inactive_bg: Color,
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
        spans.push(Span::styled(tab.label, label_style).bg(bg));
        spans.push(Span::styled("", bg));
    }
    Line::from(spans)
}
