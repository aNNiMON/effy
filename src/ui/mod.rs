use ratatui::{layout::Rect, style::Stylize as _, text::Line};
use tui_input::Input;

mod app_ui;
mod modal;
mod modal_alert;
mod modal_custom_select;
mod modal_help;
mod modal_save_as_file;
mod modal_trim;
pub mod state;
mod theme;
pub mod widget;

pub(crate) use modal::*;
pub(crate) use modal_alert::*;
pub(crate) use modal_custom_select::*;
pub(crate) use modal_help::*;
pub(crate) use modal_save_as_file::*;
pub(crate) use modal_trim::*;
pub(crate) use theme::*;

/// Detect if the terminal window is in portrait mode assuming proportions 2.2:1
fn is_portrait(area: Rect) -> bool {
    // TODO: real font size
    f32::from(area.width) < 2.2_f32 * f32::from(area.height)
}

/// Get the value and cursor position of an input widget
fn input_value_and_pos(input: &Input, width: u16) -> (String, u16) {
    let scroll = input.visual_scroll(width as usize).max(3) - 3;
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
