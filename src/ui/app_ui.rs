use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, List, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::App,
    model::Pane,
    ui::{
        is_portrait,
        widget::{InfoPane, OutputPane},
    },
};

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let portrait = is_portrait(area);
        let default_style = Style::new().dark_gray();
        let highlighted_style = default_style.white();

        let [info, main, help] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(5), Constraint::Fill(1), Constraint::Max(3)])
            .areas(area);
        {
            let style = if matches!(self.current_pane, Pane::Info) {
                highlighted_style
            } else {
                default_style
            };
            StatefulWidget::render(InfoPane::new(style), info, buf, &mut self.info_state);
        };

        let main_direction = if portrait {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };
        let [params, config] = Layout::default()
            .direction(main_direction)
            .constraints([Constraint::Fill(1), Constraint::Fill(3)])
            .areas(main);
        {
            let (style, list_sel_color) = if matches!(self.current_pane, Pane::Params) {
                (highlighted_style, Color::White)
            } else {
                (default_style, Color::Gray)
            };
            let items = self.params.iter().map(|param| {
                if param.enabled {
                    Line::from(vec![
                        Span::styled(&param.name, highlighted_style),
                        Span::raw(": "),
                        Span::styled(param.describe_value(), Style::default().yellow()),
                    ])
                } else {
                    Line::styled(param.describe(), default_style)
                }
            });
            StatefulWidget::render(
                List::new(items)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_set(symbols::border::ROUNDED)
                            .border_style(style)
                            .title_top(Line::from("Params").blue().left_aligned()),
                    )
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().fg(Color::Black).bg(list_sel_color)),
                params,
                buf,
                &mut self.params_list_state,
            )
        };

        {
            let style = if matches!(self.current_pane, Pane::Output) {
                highlighted_style
            } else {
                default_style
            };
            StatefulWidget::render(OutputPane::new(style), config, buf, &mut self.out_state);
        };

        {
            let keystyle = Style::default().green();
            let mut parts = vec![
                Span::styled(" Tab", keystyle),
                Span::raw(": switch tab  "),
                Span::styled("s/", keystyle),
                Span::styled("C", keystyle.underlined()),
                Span::styled("-s", keystyle),
                Span::raw(": render  "),
                Span::styled("↑/↓/k/j", keystyle),
                Span::raw(": navigate  "),
            ];

            if matches!(self.current_pane, Pane::Params)
                && let Some(selected) = self.params_list_state.selected()
                && let Some(param) = self.params.get(selected)
            {
                parts.append(&mut vec![
                    Span::styled("←/→/h/l", keystyle),
                    Span::raw(": toggle parameter  "),
                ]);
                if param.data.is_editable() {
                    parts.append(&mut vec![
                        Span::styled("Enter", keystyle),
                        Span::raw(": edit  "),
                    ]);
                }
            }
            parts.append(&mut vec![
                Span::styled("q/Esc", keystyle),
                Span::raw(if self.save_ongoing {
                    ": stop render"
                } else {
                    ": quit"
                }),
            ]);

            let lines = Line::from(parts);
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_set(symbols::border::ROUNDED)
                        .border_style(default_style)
                        .title_top(Line::from("Help").blue().left_aligned()),
                )
                .render(help, buf);
        };
    }
}
