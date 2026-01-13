use ratatui::{
    layout::Rect,
    style::{Color, Stylize},
    text::Line,
};
use tui_input::Input;

mod app_ui;
mod modal;
mod modal_alert;
mod modal_custom_select;
mod modal_save_as_file;
mod modal_trim;
pub mod state;
pub mod widget;

pub(crate) use modal::*;
pub(crate) use modal_alert::*;
pub(crate) use modal_custom_select::*;
pub(crate) use modal_save_as_file::*;
pub(crate) use modal_trim::*;

fn is_portrait(area: Rect) -> bool {
    // TODO: real font size
    f32::from(area.width) < 2.2_f32 * f32::from(area.height)
}

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

fn checkbox_line(checked: bool, label: &str, active: bool) -> Line<'_> {
    let mut line = Line::from(vec![
        if checked {
            "[\u{25a0}]".light_blue()
        } else {
            "[ ]".gray()
        },
        format!(" {label}").gray(),
    ]);
    if active {
        line = line.bg(Color::DarkGray);
    }
    line
}
