use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::prelude::Frame;
use ratatui::style::Stylize as _;
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear};

use crate::ui::state::InfoPaneState;
use crate::ui::widget::{InfoPane, Tab, TabStyle, tabs_line};
use crate::ui::{KeyboardHandler, ModalResult, Theme, UiModal, is_portrait};

#[derive(Debug, PartialEq, Eq)]
enum HelpTab {
    Keys,
    About,
}

#[derive(Debug)]
pub(crate) struct HelpModal<'a> {
    help_state: InfoPaneState<'a>,
    about_state: InfoPaneState<'a>,
    help_tab: HelpTab,
}

impl UiModal for HelpModal<'static> {
    fn render(&self, frame: &mut Frame, theme: &Theme) {
        let area = frame.area();
        let portrait = is_portrait(area);
        let [modal_area] = Layout::vertical([Constraint::Length(21)])
            .horizontal_margin(if portrait { 1 } else { area.width / 4 })
            .flex(Flex::Center)
            .areas(area);

        let keys_tab_active = self.help_tab == HelpTab::Keys;
        let tabs = [
            Tab {
                label: "Keys",
                active: keys_tab_active,
            },
            Tab {
                label: "About",
                active: !keys_tab_active,
            },
        ];
        let tabs_style = TabStyle::from_theme(theme);
        let block = Block::default()
            .title_top(tabs_line(&tabs, tabs_style).left_aligned())
            .borders(Borders::all())
            .border_type(BorderType::Thick)
            .border_style(theme.border_modal_style());

        frame.render_widget(Clear, modal_area);
        frame.render_stateful_widget(
            InfoPane::new(block),
            modal_area,
            &mut if keys_tab_active {
                self.help_state.clone()
            } else {
                self.about_state.clone()
            },
        );
    }
}

impl KeyboardHandler for HelpModal<'_> {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        match (&self.help_tab, key.code) {
            (_, KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::F(1) | KeyCode::Esc) => {
                return ModalResult::Close;
            }
            (HelpTab::Keys, KeyCode::Tab | KeyCode::BackTab) => self.help_tab = HelpTab::About,
            (HelpTab::Keys, KeyCode::Down | KeyCode::Char('j')) => self.help_state.scroll_down(),
            (HelpTab::Keys, KeyCode::Up | KeyCode::Char('k')) => self.help_state.scroll_up(),
            (HelpTab::About, KeyCode::Tab | KeyCode::BackTab) => self.help_tab = HelpTab::Keys,
            (HelpTab::About, KeyCode::Down | KeyCode::Char('j')) => self.about_state.scroll_down(),
            (HelpTab::About, KeyCode::Up | KeyCode::Char('k')) => self.about_state.scroll_up(),
            _ => {}
        }
        ModalResult::None
    }
}

impl<'a> HelpModal<'static> {
    pub(crate) fn new(theme: &'a Theme) -> HelpModal<'static> {
        Self {
            help_state: InfoPaneState::new(HelpBuilder::new(theme).build()),
            about_state: InfoPaneState::new(AboutBuilder::new(theme).build()),
            help_tab: HelpTab::Keys,
        }
    }
}

struct HelpBuilder<'a> {
    theme: &'a Theme,
}

impl<'a> HelpBuilder<'a> {
    fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn build(&self) -> Text<'static> {
        let mut lines = Vec::new();
        lines.push(Line::from("         Key Action".fg(self.theme.pane_title_color())).bold());
        lines.extend(self.navigation_lines());
        lines.extend(self.render_lines());
        lines.extend(self.modals_lines());
        lines.extend(self.clipboard_lines());
        Text::from(lines)
    }

    fn navigation_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        lines.push(
            Line::from(" Navigation:")
                .fg(self.theme.pane_title_color())
                .bold(),
        );
        lines.extend(self.lines(
            &["↑", "k"],
            "Scroll up in the Info, Parameter or Output pane",
        ));
        lines.extend(self.lines(
            &["↓", "j"],
            "Scroll down in the Info, Parameter or Output pane",
        ));
        lines.extend(self.lines(&["Tab"], "Focus next pane/tab"));
        lines.extend(self.lines(&["Shift+Tab"], "Focus previous pane/tab"));
        lines.extend(self.lines(&["i"], "Focus Info pane"));
        lines.extend(self.lines(
            &["←", "h"],
            "Switch the previous quick option in the Parameter pane",
        ));
        lines.extend(self.lines(
            &["→", "l"],
            "Select the next quick option in the Parameter pane",
        ));
        lines.extend(self.lines(&["Enter"], "Open parameter options"));
        lines.extend(self.lines(&["Esc", "q", "Ctrl+c"], "Quit the application"));
        lines.extend(self.lines(&["o"], "Focus Output pane"));
        lines.extend(self.lines(&["?", "F1"], "Toggle help"));
        lines
    }

    fn render_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(
            Line::from(" Render:")
                .fg(self.theme.pane_title_color())
                .bold(),
        );
        lines.extend(self.lines(&["s"], "Open the 'Render As' modal"));
        lines.extend(self.lines(&["Ctrl+s"], "Quick render"));
        lines.extend(self.lines(
            &["Esc", "q", "Ctrl+c"],
            "Stop rendering if it's in progress",
        ));
        lines
    }

    fn modals_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(
            Line::from(" Modals:")
                .fg(self.theme.pane_title_color())
                .bold(),
        );
        lines.extend(self.lines(&["Tab"], "Focus next field"));
        lines.extend(self.lines(&["Shift+Tab"], "Focus previous field"));
        lines.extend(self.lines(&["Space"], "Toggle a checkbox"));
        lines.extend(self.lines(&["Esc"], "Close an active modal"));
        lines
    }

    fn clipboard_lines(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(
            Line::from(" Clipboard:")
                .fg(self.theme.pane_title_color())
                .bold(),
        );
        lines.extend(self.lines(&["p"], "Copy a preset to clipboard"));
        lines.extend(self.lines(&["y"], "Copy a FFmpeg command to clipboard"));
        lines
    }

    fn lines(&self, keys: &'static [&str], v: &'static str) -> Vec<Line<'static>> {
        let key_style = self.theme.key_style();
        let text_style = self.theme.text_color();
        let repeated_style = self.theme.text_muted_color();

        keys.iter()
            .enumerate()
            .map(|(i, k)| {
                Line::from(vec![
                    Span::styled(format!("{k: >12} "), key_style),
                    Span::styled(
                        v.to_owned(),
                        if i == 0 { text_style } else { repeated_style },
                    ),
                ])
            })
            .collect()
    }
}

struct AboutBuilder<'a> {
    theme: &'a Theme,
}

impl<'a> AboutBuilder<'a> {
    fn new(theme: &'a Theme) -> Self {
        Self { theme }
    }

    fn build(&self) -> Text<'static> {
        let mut lines = Vec::new();
        lines.extend(self.logo());
        lines.extend(self.links());
        lines.extend(self.ratatui_logo());
        Text::from(lines)
    }

    fn logo(&self) -> Vec<Line<'static>> {
        let color = self.theme.logo_color();
        let mut lines: Vec<Line> = vec![];
        let logo = [
            "",
            "           /████████ /████████       ",
            "          | ██_____/| ██_____/       ",
            "  /██████ | ██      | ██    /██   /██",
            " /██__  ██| █████   | █████| ██  | ██",
            "| ████████| ██__/   | ██__/| ██  | ██",
            "| ██_____/| ██      | ██   | ██  | ██",
            "|  ███████| ██      | ██   |  ███████",
            r" \_______/|__/      |__/    \____  ██",
            "                            /██  | ██",
        ];
        for line in logo {
            lines.push(Line::from(line).fg(color).centered());
        }
        lines.push(
            Line::from(format!(
                "               v{}      |  ██████/",
                env!("CARGO_PKG_VERSION")
            ))
            .fg(color)
            .centered(),
        );
        lines.push(
            Line::from(r"                            \______/ ")
                .fg(color)
                .centered(),
        );
        lines
    }

    fn ratatui_logo(&self) -> Vec<Line<'static>> {
        let mut lines: Vec<Line> = vec![Line::default()];
        let logo = [
            "Built with Rust. Powered by".fg(self.theme.about_caption_color()),
            "█▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █".fg(self.theme.ratatui_logo_color()),
            "█▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █".fg(self.theme.ratatui_logo_color()),
        ];
        for line in logo {
            lines.push(Line::from(line).centered());
        }
        lines
    }

    fn links(&self) -> Vec<Line<'static>> {
        let link = self.theme.link_style();
        vec![
            Line::default(),
            Line::from(Span::styled("https://github.com/aNNiMON/effy", link)).centered(),
        ]
    }
}
