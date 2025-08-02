use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::{App, Pane};

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let default_style = Style::new().white();
        let highlighted_style = default_style.blue();

        let [info, main] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(5), Constraint::Fill(1)])
            .areas(area);
        {
            let style = if matches!(self.current_pane, Pane::Info) {
                highlighted_style
            } else {
                default_style
            };
            Paragraph::new(self.info_text.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(style)
                        .title_top(Line::from(" effy ").bold().blue().centered())
                        .title_top(Line::from(" Info ").blue().left_aligned()),
                )
                .scroll((self.info_pane_current_line, 0))
                .render(info, buf);
        }

        let [params, config] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1), Constraint::Fill(3)])
            .areas(main);
        {
            let style = if matches!(self.current_pane, Pane::Params) {
                highlighted_style
            } else {
                default_style
            };
            Paragraph::new("Params")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(style)
                        .title_top(Line::from(" Params ").blue().left_aligned()),
                )
                .render(params, buf);
        }

        {
            let style = if matches!(self.current_pane, Pane::Config) {
                highlighted_style
            } else {
                default_style
            };
            Paragraph::new("Config area")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(style)
                        .title_top(Line::from(" Config ").blue().left_aligned())
                        .title_bottom(Line::from(" Press 'q' to quit ").dim().left_aligned()),
                )
                .render(config, buf);
        }
    }
}
