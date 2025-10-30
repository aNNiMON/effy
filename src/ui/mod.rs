use ratatui::{
    style::{Color, Stylize},
    text::Line,
};
use tui_input::Input;

mod app_ui;
mod modal;
mod modal_save_as_file;
mod modal_trim;

pub(crate) use modal::*;
pub(crate) use modal_save_as_file::*;
pub(crate) use modal_trim::*;

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
            "[■]".green()
        } else {
            "[ ]".gray()
        },
        format!(" {}", label).gray(),
    ]);
    if active {
        line = line.bg(Color::DarkGray);
    }
    line
}
