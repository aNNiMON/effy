use ratatui::{
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{
        Block, Borders, List, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};

use crate::{app::App, model::Pane};

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
            let (style, list_sel_color) = if matches!(self.current_pane, Pane::Params) {
                (highlighted_style, Color::Blue)
            } else {
                (default_style, Color::Gray)
            };
            let items = self.params.iter().map(|(enabled, param)| {
                Line::from(param.describe()).style(if *enabled {
                    highlighted_style
                } else {
                    default_style.dim()
                })
            });
            StatefulWidget::render(
                List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(style)
                            .title_top(Line::from(" Params ").blue().left_aligned()),
                    )
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().bold().fg(Color::Black).bg(list_sel_color)),
                params,
                buf,
                &mut self.params_list_state.clone(),
            );
        }

        {
            let style = if matches!(self.current_pane, Pane::Output) {
                highlighted_style
            } else {
                default_style
            };

            let output_lines = self.output.lines().count() as u16;
            let pane_height = config.height.saturating_sub(2);
            let max_length = output_lines.saturating_sub(pane_height);
            let offset = if output_lines > pane_height {
                max_length
                    .saturating_sub(self.output_pane_current_line)
                    .min(max_length)
            } else {
                0
            };

            Paragraph::new(self.output.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(style)
                        .title_top(Line::from(" Output ").blue().left_aligned())
                        .title_bottom(Line::from(" [ctrl+s] process / [q] quit ").dim()),
                )
                .scroll((offset, 0))
                .render(config, buf);

            if output_lines > pane_height {
                let mut scrollbar_state =
                    ScrollbarState::new(max_length as usize).position(offset as usize);
                Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓"))
                    .render(
                        config.inner(Margin {
                            vertical: 1,
                            horizontal: 0,
                        }),
                        buf,
                        &mut scrollbar_state,
                    );
            }
        }
    }
}
