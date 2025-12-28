use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Style, Stylize as _},
    symbols,
    text::Line,
    widgets::{
        Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
        Widget,
    },
};

use crate::ui::state::OutputPaneState;

pub struct OutputPane {
    pub border_style: Style,
}

impl OutputPane {
    pub fn new(border_style: Style) -> OutputPane {
        OutputPane { border_style }
    }
}

impl StatefulWidget for OutputPane {
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

        Paragraph::new(state.output.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_set(symbols::border::ROUNDED)
                    .border_style(self.border_style)
                    .title_top(Line::from("Output").blue().left_aligned()),
            )
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
