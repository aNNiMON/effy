use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    widgets::{
        Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget, Widget,
    },
};

use crate::ui::state::OutputPaneState;

// Scrollable String widget with ScrollBar

#[derive(Debug)]
pub struct OutputPane<'a> {
    pub block: Block<'a>,
}

impl OutputPane<'_> {
    pub fn new(block: Block<'_>) -> OutputPane<'_> {
        OutputPane { block }
    }
}

impl StatefulWidget for OutputPane<'_> {
    type State = OutputPaneState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let output_lines = state.output.lines().count() as u16;
        let pane_height = area.height.saturating_sub(2);
        let max_length = output_lines.saturating_sub(pane_height);
        let offset = if output_lines > pane_height {
            max_length
                .saturating_sub(state.current_line)
                .min(max_length)
        } else {
            0
        };
        state.current_line = state.current_line.min(max_length);

        Paragraph::new(state.output.clone())
            .block(self.block)
            .scroll((offset, 0))
            .render(area, buf);

        if output_lines > pane_height {
            let mut scrollbar_state =
                ScrollbarState::new(max_length as usize).position(offset as usize);
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"))
                .render(
                    area.inner(Margin {
                        vertical: 1,
                        horizontal: 0,
                    }),
                    buf,
                    &mut scrollbar_state,
                );
        }
    }
}
