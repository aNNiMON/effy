use crossterm::event::{KeyCode, KeyEvent};
use ratatui::layout::{Constraint, Flex, Layout, Offset};
use ratatui::prelude::Frame;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Borders, Clear, RatatuiLogo, Widget as _};

use crate::ui::state::InfoPaneState;
use crate::ui::widget::{InfoPane, Tab, TabStyle, tabs_line};
use crate::ui::{KeyboardHandler, ModalResult, UiModal, is_portrait};

#[derive(PartialEq, Eq)]
enum HelpTab {
    KeyBindings,
    About,
}

pub(crate) struct HelpModal<'a> {
    help_state: InfoPaneState<'a>,
    help_tab: HelpTab,
    effy_logo: Text<'a>,
    ratatui_logo: RatatuiLogo,
}

impl UiModal for HelpModal<'static> {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let portrait = is_portrait(area);
        let [modal_area] = Layout::vertical([Constraint::Length(17)])
            .horizontal_margin(if portrait { 1 } else { area.width / 4 })
            .flex(Flex::Center)
            .areas(area);

        let keys_tab_active = self.help_tab == HelpTab::KeyBindings;
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

        Clear.render(modal_area, frame.buffer_mut());
        if keys_tab_active {
            frame.render_stateful_widget(
                InfoPane::new(block),
                modal_area,
                &mut self.help_state.clone(),
            );
        } else {
            let [logo_area] = Layout::horizontal([Constraint::Length(27)])
                .flex(Flex::Center)
                .areas(modal_area);
            frame.render_widget(block, modal_area);
            frame.render_widget(&self.effy_logo, modal_area);
            let logo_area = logo_area.offset(Offset::new(0, 1 + self.effy_logo.height() as i32));
            frame.render_widget("Built with Rust. Powered by".yellow(), logo_area);
            let logo_area = logo_area.offset(Offset::new(0, 1));
            frame.render_widget(self.ratatui_logo, logo_area);
        }
    }
}

impl KeyboardHandler for HelpModal<'_> {
    fn handle_key(&mut self, key: KeyEvent) -> ModalResult {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::F(1) | KeyCode::Esc => {
                return ModalResult::Close;
            }
            KeyCode::Tab | KeyCode::BackTab => {
                self.help_tab = match self.help_tab {
                    HelpTab::KeyBindings => HelpTab::About,
                    HelpTab::About => HelpTab::KeyBindings,
                }
            }
            KeyCode::Down | KeyCode::Char('j') => self.help_state.scroll_down(),
            KeyCode::Up | KeyCode::Char('k') => self.help_state.scroll_up(),
            _ => {}
        }
        ModalResult::None
    }
}

impl<'a> HelpModal<'a> {
    pub(crate) fn new() -> HelpModal<'a> {
        Self {
            help_state: InfoPaneState::new(Self::help_lines()),
            help_tab: HelpTab::KeyBindings,
            effy_logo: Text::from(Self::logo()),
            ratatui_logo: RatatuiLogo::small(),
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
        let style = Style::default().green().on_black();
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
                r"             by aNNiMON     \______/ ",
                style,
            ))
            .centered(),
        );
        lines
    }
}
