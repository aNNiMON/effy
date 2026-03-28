use ratatui::layout::Rect;
use ratatui::style::Stylize as _;
use ratatui::text::Line;
use tui_input::Input;

mod app_ui;
pub mod modal;
pub mod state;
mod theme;
pub mod widget;

pub(crate) use theme::*;

/// Detect if the terminal window is in portrait mode assuming proportions 2.2:1
fn is_portrait(area: Rect) -> bool {
    // TODO: real font size
    f32::from(area.width) < 2.2_f32 * f32::from(area.height)
}

/// Get the value and cursor position of an input widget
fn input_value_and_pos(input: &Input, width: u16) -> (String, u16) {
    let scroll = input.visual_scroll(width as usize);
    let value = input
        .value()
        .chars()
        .skip(scroll)
        .take(width as usize)
        .collect::<String>();
    let pos = input.visual_cursor().max(scroll) - scroll + 1;
    (value, pos as u16)
}

/// Prepare a checkbox Line
fn checkbox_line<'a>(checked: bool, label: &str, active: bool, theme: &'a Theme) -> Line<'a> {
    let mut line = Line::from(vec![
        if checked {
            "[\u{25a0}]".fg(theme.checkbox_checked_color())
        } else {
            "[ ]".fg(theme.checkbox_color())
        },
        format!(" {label}").fg(theme.checkbox_label_color()),
    ]);
    if active {
        line = line.patch_style(theme.checkbox_focused_style());
    }
    line
}
