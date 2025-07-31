use ratatui::{
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

use crate::app::App;

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" effy ").bold().blue().centered();
        let text = "Input file: abc.mp4\n\
            File size: 1.1MiB\n\
            Duration: 05:16";
        let paragraph = Paragraph::new(text).block(Block::bordered().title(title));
        paragraph.render(area, buf);
    }
}
