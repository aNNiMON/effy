use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Clear, Widget},
};

// Clear background using specified color

#[derive(Debug)]
pub struct BgClear {
    style: Style,
    fill: bool,
}

impl BgClear {
    pub fn new(color: Color) -> BgClear {
        BgClear {
            style: Style::default().bg(color),
            fill: color != Color::Reset,
        }
    }
}

impl Widget for BgClear {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        if self.fill {
            Block::default().style(self.style).render(area, buf);
        }
    }
}
