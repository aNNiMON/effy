use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Flex, Layout, Margin, Position, Rect},
    style::{Color, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, List, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};
use tui_input::Input;

use crate::{app::App, model::Modal, model::Pane};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
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
                Span::styled("C", keystyle.underlined()),
                Span::styled("-s", keystyle),
                Span::raw(": render  "),
                Span::styled("↑/↓/k/j", keystyle),
                Span::raw(": navigate  "),
            ];
            if matches!(self.current_pane, Pane::Params) {
                lines.append(&mut vec![
                    Span::styled("←/→/h/l", keystyle),
                    Span::raw(": toggle parameter  "),
                ]);
            }
            lines.append(&mut vec![
                Span::styled("q/Esc", keystyle),
                Span::raw(": quit"),
            ]);

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
                let [modal_area] = Layout::vertical([Constraint::Length(6)])
                    .horizontal_margin(area.width / 5)
                    .flex(Flex::Center)
                    .areas(area);
                let [input_area, hints_area] =
                    Layout::vertical([Constraint::Length(3), Constraint::Length(1)])
                        .horizontal_margin(2)
                        .vertical_margin(1)
                        .areas(modal_area);

                let width = input_area.width.max(3) - 3;
                let scroll = input.visual_scroll(width as usize);
                let display_value = input
                    .value()
                    .chars()
                    .skip(scroll)
                    .take(width as usize)
                    .collect::<String>();

                Clear.render(modal_area, frame.buffer_mut());
                Block::bordered()
                    .border_set(symbols::border::THICK)
                    .title("Render as")
                    .fg(Color::Blue)
                    .render(modal_area, frame.buffer_mut());
                Paragraph::new(display_value)
                    .style(Style::new().white())
                    .block(Block::bordered().gray().dim())
                    .render(input_area, frame.buffer_mut());
                Self::render_input_hints(hints_area, frame);

                let x = input.visual_cursor().max(scroll) - scroll + 1;
                frame.set_cursor_position(Position {
                    x: input_area.x + x as u16,
                    y: input_area.y + 1,
                });
            }
            Modal::Trim(trim_view) => {
                let area = frame.area();
                let [modal_area] = Layout::vertical([Constraint::Length(8)])
                    .horizontal_margin(area.width / 5)
                    .flex(Flex::Center)
                    .areas(area);
                let [inputs_area, chackbox_area, hints_area] = Layout::vertical([
                    Constraint::Length(3),
                    Constraint::Length(2),
                    Constraint::Length(1),
                ])
                .flex(Flex::SpaceBetween)
                .areas(modal_area.inner(Margin::new(2, 1)));

                let [ss_area, to_area] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                        .areas(inputs_area);
                let [precise_area, use_to_area] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)])
                        .areas(chackbox_area);

                let (ss_value, ss_x) = Self::input_value_and_pos(&trim_view.ss, ss_area.width);
                let (to_value, to_x) = Self::input_value_and_pos(&trim_view.to, to_area.width);

                let active_border_style = Style::new().blue();
                let inactive_border_style = Style::new().gray();

                Clear.render(modal_area, frame.buffer_mut());
                Block::bordered()
                    .border_set(symbols::border::THICK)
                    .title("Trim")
                    .fg(Color::Blue)
                    .render(modal_area, frame.buffer_mut());

                let mut borders = [inactive_border_style; 4];
                borders[trim_view.active_input] = active_border_style;
                // Inputs
                Paragraph::new(ss_value)
                    .block(Block::bordered().style(borders[0]).title("Start".blue()))
                    .render(ss_area, frame.buffer_mut());
                let to_title = if trim_view.use_to { "To" } else { "Duration" };
                Paragraph::new(to_value)
                    .block(Block::bordered().style(borders[1]).title(to_title.blue()))
                    .render(to_area, frame.buffer_mut());
                if trim_view.active_input <= 1 {
                    let (x, y) = if trim_view.active_input == 0 {
                        (ss_area.x + ss_x, ss_area.y + 1)
                    } else {
                        (to_area.x + to_x, to_area.y + 1)
                    };
                    frame.set_cursor_position(Position { x, y });
                }
                // Checkboxes
                let precise_line =
                    Self::checkbox_line(trim_view.precise, "Precise", trim_view.active_input == 2);
                Paragraph::new(precise_line)
                    .alignment(Alignment::Center)
                    .render(precise_area, frame.buffer_mut());
                let use_to_line = Self::checkbox_line(
                    trim_view.use_to,
                    "Use Duration/To",
                    trim_view.active_input == 3,
                );
                Paragraph::new(use_to_line)
                    .alignment(Alignment::Center)
                    .render(use_to_area, frame.buffer_mut());
                Self::render_input_hints(hints_area, frame);
            }
        }
    }

    fn render_input_hints(area: Rect, frame: &mut Frame) {
        let parts = Line::from(vec![
            "Enter".gray().bold(),
            ": confirm  ".gray(),
            "Esc".gray().bold(),
            ": close".gray(),
        ]);
        Paragraph::new(parts).render(area, frame.buffer_mut());
    }

    fn input_value_and_pos(input: &Input, width: u16) -> (String, u16) {
        let scroll = input.visual_scroll(width as usize).max(3) - 3;
        let display_value = input
            .value()
            .chars()
            .skip(scroll)
            .take(width as usize)
            .collect::<String>();
        let pos = input.visual_cursor().max(scroll) - scroll + 1;
        (display_value, pos as u16)
    }

    fn checkbox_line(checked: bool, label: &str, active: bool) -> Line<'_> {
        let mut line = Line::from(vec![
            if checked {
                "[■]".green()
            } else {
                "[ ]".gray()
            },
            format!(" {}", label).gray(),
        ]);
        if active {
            line = line.bg(Color::DarkGray);
        }
        line
    }
}
