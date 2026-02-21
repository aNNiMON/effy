use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::ui::state::InfoPaneState;

// Scrollable Text widget

#[derive(Debug)]
pub struct InfoPane<'a> {
    pub block: Block<'a>,
}

impl<'a> InfoPane<'a> {
    pub fn new(block: Block<'a>) -> InfoPane<'a> {
        InfoPane { block }
    }
}

impl<'a> StatefulWidget for InfoPane<'a> {
    type State = InfoPaneState<'a>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        Paragraph::new(state.text.clone())
            .block(self.block)
            .scroll((state.current_line, 0))
            .render(area, buf);
    }
}
