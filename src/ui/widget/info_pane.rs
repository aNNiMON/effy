use std::marker::PhantomData;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize as _},
    symbols,
    text::Line,
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::ui::state::InfoPaneState;

pub struct InfoPane<'a> {
    pub border_style: Style,
    marker: PhantomData<&'a ()>,
}

impl<'a> InfoPane<'a> {
    pub fn new(border_style: Style) -> InfoPane<'a> {
        InfoPane {
            border_style,
            marker: PhantomData,
        }
    }
}

impl<'a> StatefulWidget for InfoPane<'a> {
    type State = InfoPaneState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Paragraph::new(state.text.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .border_style(self.border_style)
                    .title_top(Line::from("effy").bold().blue().centered())
                    .title_top(Line::from("Info").blue().left_aligned()),
            )
            .scroll((state.current_line, 0))
            .render(area, buf);
    }
}
