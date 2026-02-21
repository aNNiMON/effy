use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Flex, Layout};
use ratatui::prelude::Frame;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear};

use crate::ui::state::InfoPaneState;
use crate::ui::widget::{InfoPane, Tab, TabStyle, tabs_line};
use crate::ui::{KeyboardHandler, ModalResult, UiModal, is_portrait};

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
    fn render(&self, frame: &mut Frame) {
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
        let tabs_style = TabStyle {
            active_style: Style::default().white().bold(),
            inactive_style: Style::default().gray(),
            active_bg: Color::Blue,
            inactive_bg: Color::Black,
        };

        let block = Block::default()
            .title_top(tabs_line(&tabs, tabs_style).left_aligned())
            .borders(Borders::all())
            .border_type(BorderType::Thick)
            .border_style(Color::Blue);

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

impl<'a> HelpModal<'a> {
    pub(crate) fn new() -> HelpModal<'a> {
        Self {
            help_state: InfoPaneState::new(Self::help_lines()),
            about_state: InfoPaneState::new(Self::about_lines()),
            help_tab: HelpTab::Keys,
        }
    }

    fn help_lines() -> Text<'a> {
        let mut lines = Vec::new();
        lines.push(Line::from("         Key Action").blue().bold());
        lines.extend(Self::help_navigation_lines());
        lines.extend(Self::help_render_lines());
        lines.extend(Self::help_modals_lines());
        lines.extend(Self::help_clipboard_lines());
        Text::from(lines)
    }

    fn help_navigation_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::from(" Navigation:").blue().bold());
        lines.extend(Self::lines(
            &["↑", "k"],
            "Scroll up in the Info, Parameter or Output pane",
        ));
        lines.extend(Self::lines(
            &["↓", "j"],
            "Scroll down in the Info, Parameter or Output pane",
        ));
        lines.extend(Self::lines(&["Tab"], "Focus next pane/tab"));
        lines.extend(Self::lines(&["Shift+Tab"], "Focus previous pane/tab"));
        lines.extend(Self::lines(&["i"], "Focus Info pane"));
        lines.extend(Self::lines(
            &["←", "h"],
            "Switch the previous quick option in the Parameter pane",
        ));
        lines.extend(Self::lines(
            &["→", "l"],
            "Select the next quick option in the Parameter pane",
        ));
        lines.extend(Self::lines(&["Enter"], "Open parameter options"));
        lines.extend(Self::lines(&["Esc", "q", "Ctrl+c"], "Quit the application"));
        lines.extend(Self::lines(&["o"], "Focus Output pane"));
        lines.extend(Self::lines(&["?", "F1"], "Toggle help"));
        lines
    }

    fn help_render_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(Line::from(" Render:").blue().bold());
        lines.extend(Self::lines(&["s"], "Open the 'Render As' modal"));
        lines.extend(Self::lines(&["Ctrl+s"], "Quick render"));
        lines.extend(Self::lines(
            &["Esc", "q", "Ctrl+c"],
            "Stop rendering if it's in progress",
        ));
        lines
    }

    fn help_modals_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(Line::from(" Modals:").blue().bold());
        lines.extend(Self::lines(&["Tab"], "Focus next field"));
        lines.extend(Self::lines(&["Shift+Tab"], "Focus previous field"));
        lines.extend(Self::lines(&["Space"], "Toggle a checkbox"));
        lines.extend(Self::lines(&["Esc"], "Close an active modal"));
        lines
    }

    fn help_clipboard_lines() -> Vec<Line<'a>> {
        let mut lines = Vec::new();
        lines.push(Line::default());
        lines.push(Line::from(" Clipboard:").blue().bold());
        lines.extend(Self::lines(&["p"], "Copy a preset to clipboard"));
        lines.extend(Self::lines(&["y"], "Copy a FFmpeg command to clipboard"));
        lines
    }

    fn lines(keys: &'a [&str], v: &'a str) -> Vec<Line<'a>> {
        let key_style = Style::default().green();
        let text_style = Style::default().gray();
        let repeated_style = Style::default().dark_gray();

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

    fn logo() -> Vec<Line<'a>> {
        let style = Style::default().green();
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
            lines.push(Line::from(Span::styled(line, style)).centered());
        }
        lines.push(
            Line::from(Span::styled(
                format!(
                    "               v{}      |  ██████/",
                    env!("CARGO_PKG_VERSION")
                ),
                style,
            ))
            .centered(),
        );
        lines.push(
            Line::from(Span::styled(
                r"                            \______/ ",
                style,
            ))
            .centered(),
        );
        lines
    }

    fn ratatui_logo() -> Vec<Line<'a>> {
        let mut lines: Vec<Line> = vec![Line::default()];
        let logo = [
            "Built with Rust. Powered by".yellow(),
            "█▀▀▄ ▄▀▀▄▝▜▛▘▄▀▀▄▝▜▛▘█  █ █".white(),
            "█▀▀▄ █▀▀█ ▐▌ █▀▀█ ▐▌ ▀▄▄▀ █".white(),
        ];
        for line in logo {
            lines.push(Line::from(line).centered());
        }
        lines
    }

    fn links() -> Vec<Line<'a>> {
        let link = Style::default()
            .light_blue()
            .underlined()
            .underline_color(Color::LightBlue);
        vec![
            Line::default(),
            Line::from(Span::styled("https://github.com/aNNiMON/effy", link)).centered(),
        ]
    }

    fn about_lines() -> Text<'a> {
        let mut lines = Vec::new();
        lines.extend(Self::logo());
        lines.extend(Self::links());
        lines.extend(Self::ratatui_logo());
        Text::from(lines)
    }
}
