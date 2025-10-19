use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Margin, Position},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};

use crate::{app::App, model::Modal, model::Pane};

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
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
            Paragraph::new(self.info_text.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_set(symbols::border::ROUNDED)
                        .border_style(style)
                        .title_top(Line::from("effy").bold().blue().centered())
                        .title_top(Line::from("Info").blue().left_aligned()),
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
                (highlighted_style, Color::White)
            } else {
                (default_style, Color::Gray)
            };
            let items = self.params.iter().map(|param| {
                if param.enabled {
                    Line::from(vec![
                        Span::styled(param.name.clone(), highlighted_style),
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
                        .border_set(symbols::border::ROUNDED)
                        .border_style(style)
                        .title_top(Line::from("Output").blue().left_aligned()),
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

        {
            let keystyle = Style::default().green();
            let mut lines = vec![
                Span::styled(" Tab", keystyle),
                Span::raw(": switch tab  "),
                Span::styled(" C-s", keystyle),
                Span::raw(": render  "),
                Span::styled(" ↑/↓/k/j", keystyle),
                Span::raw(": navigate  "),
            ];
            if matches!(self.current_pane, Pane::Params) {
                lines.push(Span::styled("←/→/h/l", keystyle));
                lines.push(Span::raw(": toggle parameter  "));
            }
            lines.push(Span::styled("q/Esc", keystyle));
            lines.push(Span::raw(": quit"));

            let lines = Line::from(lines);
            Paragraph::new(lines)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_set(symbols::border::ROUNDED)
                        .border_style(default_style)
                        .title_top(Line::from("Help").blue().left_aligned()),
                )
                .render(help, buf);
        }
    }
}

impl Modal {
    pub(crate) fn render(&self, frame: &mut ratatui::prelude::Frame) {
        match self {
            Modal::SaveFileAs(input) => {
                let area = frame.area();
                let [input_area] = Layout::vertical([Constraint::Length(3)])
                    .horizontal_margin(area.width / 5)
                    .flex(Flex::Center)
                    .areas(area);

                let block = Block::bordered()
                    .border_set(symbols::border::THICK)
                    .title("New filename")
                    .fg(Color::Blue);
                Clear.render(input_area, frame.buffer_mut());

                let width = input_area.width.max(3) - 3;
                let scroll = input.visual_scroll(width as usize);
                let display_value = input
                    .value()
                    .chars()
                    .skip(scroll)
                    .take(width as usize)
                    .collect::<String>();

                Paragraph::new(display_value)
                    .style(Style::new().white())
                    .block(block)
                    .render(input_area, frame.buffer_mut());

                let x = input.visual_cursor().max(scroll) - scroll + 1;
                frame.set_cursor_position(Position {
                    x: input_area.x + x as u16,
                    y: input_area.y + 1,
                });
            }
        }
    }
}
