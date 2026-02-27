use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize as _,
    symbols,
    text::{Line, Span},
    widgets::{Block, Borders, List, StatefulWidget, Widget},
};

use crate::{
    app::App,
    model::Pane,
    ui::{
        is_portrait,
        widget::{InfoPane, OutputPane, Tab, TabStyle, tabs_line},
    },
};

impl Widget for &mut App<'_> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let portrait = is_portrait(area);
        let default_style = self.theme.border_inactive_style();
        let highlighted_style = self.theme.border_active_style();

        let [main, help] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Max(1)])
            .areas(area);

        let main_direction = if portrait {
            Direction::Vertical
        } else {
            Direction::Horizontal
        };
        let [params, output] = Layout::default()
            .direction(main_direction)
            .constraints([Constraint::Min(5), Constraint::Fill(3)])
            .areas(main);
        // Params pane
        {
            let style = if matches!(self.current_pane, Pane::Params) {
                highlighted_style
            } else {
                default_style
            };
            let items = self.params.iter().map(|param| {
                if param.enabled {
                    Line::from(vec![
                        Span::raw(&param.name),
                        Span::raw(": "),
                        Span::styled(param.describe_value(), self.theme.text_param_color()),
                    ])
                } else {
                    Line::styled(param.describe(), self.theme.text_param_disabled_color())
                }
            });
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .border_style(style)
                .title_top(
                    Line::from("Params")
                        .fg(self.theme.pane_title_color())
                        .left_aligned(),
                );
            if portrait {
                block = block.title_top(
                    Line::from("effy")
                        .fg(self.theme.pane_title_color())
                        .bold()
                        .centered(),
                );
            }
            StatefulWidget::render(
                List::new(items)
                    .block(block)
                    .style(highlighted_style)
                    .highlight_style(self.theme.list_highlight_style()),
                params,
                buf,
                &mut self.params_list_state,
            )
        };

        // Info/Output pane
        {
            let (border_style, tab_color) =
                if matches!(self.current_pane, Pane::Info | Pane::Output) {
                    (highlighted_style, self.theme.tab_bg_active())
                } else {
                    (default_style, self.theme.tab_bg_inactive())
                };
            let info_active = matches!(self.active_out_pane, Pane::Info);
            let tabs = [
                Tab {
                    label: "Info",
                    active: info_active,
                },
                Tab {
                    label: "Output",
                    active: !info_active,
                },
            ];
            let mut tabs_style = TabStyle::from_theme(&self.theme);
            tabs_style.active_bg = tab_color;
            let mut block = Block::default()
                .borders(Borders::ALL)
                .border_set(symbols::border::ROUNDED)
                .border_style(border_style)
                .title_top(tabs_line(&tabs, tabs_style).left_aligned());
            if !portrait {
                block = block.title_top(
                    Line::from("effy")
                        .fg(self.theme.pane_title_color())
                        .bold()
                        .centered(),
                );
            }

            if matches!(self.active_out_pane, Pane::Output) {
                OutputPane::new(block).render(output, buf, &mut self.out_state);
            } else {
                InfoPane::new(block).render(output, buf, &mut self.info_state);
            }
        };

        // Help bar
        {
            let key_style = self.theme.key_style();
            let raw_style = self.theme.text_color();
            let mut parts = vec![
                Span::styled("Tab", key_style),
                Span::styled(": switch tab  ", raw_style),
                Span::styled("s/", key_style),
                Span::styled("C", key_style.underlined()),
                Span::styled("-s", key_style),
                Span::styled(": render  ", raw_style),
                Span::styled("↑/↓/k/j", key_style),
                Span::styled(": navigate  ", raw_style),
            ];

            if matches!(self.current_pane, Pane::Params)
                && let Some(selected) = self.params_list_state.selected()
                && let Some(param) = self.params.get(selected)
            {
                parts.append(&mut vec![
                    Span::styled("←/→/h/l", key_style),
                    Span::styled(": toggle parameter  ", raw_style),
                ]);
                if param.data.is_editable() {
                    parts.append(&mut vec![
                        Span::styled("Enter", key_style),
                        Span::styled(": edit  ", raw_style),
                    ]);
                }
            }
            parts.append(&mut vec![
                Span::styled("q/Esc", key_style),
                Span::styled(
                    if self.save_ongoing {
                        ": stop render  "
                    } else {
                        ": quit  "
                    },
                    raw_style,
                ),
                Span::styled("F1/?", key_style),
                Span::styled(": help", raw_style),
            ]);

            Line::from(parts).render(help, buf);
        };
    }
}
