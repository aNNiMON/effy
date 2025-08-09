use std::process::{Command, Stdio};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::ListState};

use crate::model::{AudioBitrate, Pane, Param, Parameter, VideoBitrate};

pub(crate) struct App {
    pub(crate) running: bool,
    pub(crate) current_pane: Pane,
    pub(crate) input_file: String,
    pub(crate) info_text: String,
    pub(crate) info_pane_current_line: u16,
    pub(crate) output: String,
    pub(crate) params: Vec<Param>,
    pub(crate) params_list_state: ListState,
}

impl App {
    pub fn new(input_file: String) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            running: false,
            current_pane: Pane::Params,
            input_file: input_file.clone(),
            info_text: format!(
                "Input file: {}\n\
                 File size: 1.1MiB\n\
                 Duration: 05:16\n\
                 Has Audio: Yes\n\
                 Has Video: Yes\n\
                 Video Codec: h264\n\
                 Audio Codec: aac\n\
                 Video Resolution: 1920x1080\n\
                 Video Bitrate: 4500kbps\n\
                 Audio Bitrate: 192kbps\n\
                 Video FPS: 30\n\
                 Audio Channels: 2",
                input_file
            ),
            info_pane_current_line: 0,
            output: "".to_string(),
            params: vec![
                Param::DisableAudio(false),
                Param::AudioBitrate(AudioBitrate::Auto),
                Param::VideoBitrate(VideoBitrate::Auto),
            ],
            params_list_state: list_state,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (self.current_pane, key.modifiers, key.code) {
            (_, _, KeyCode::Esc | KeyCode::Char('q'))
            | (_, KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, _, KeyCode::Tab) => self.next_pane(),
            (_, KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (Pane::Info, _, KeyCode::Down | KeyCode::Char('j')) => self.scroll_info_pane_down(),
            (Pane::Info, _, KeyCode::Up | KeyCode::Char('k')) => self.scroll_info_pane_up(),
            (Pane::Params, _, KeyCode::Down | KeyCode::Char('j')) => self.select_next_param(),
            (Pane::Params, _, KeyCode::Up | KeyCode::Char('k')) => self.select_prev_param(),
            (Pane::Params, _, KeyCode::Left | KeyCode::Char('h')) => self.prev_option(),
            (Pane::Params, _, KeyCode::Right | KeyCode::Char('l')) => self.next_option(),
            _ => {}
        }
    }

    fn scroll_info_pane_down(&mut self) {
        if self.info_pane_current_line < self.info_text.lines().count() as u16 - 1 {
            self.info_pane_current_line += 1;
        }
    }

    fn scroll_info_pane_up(&mut self) {
        self.info_pane_current_line = self.info_pane_current_line.saturating_sub(1);
    }

    fn select_prev_param(&mut self) {
        if let Some(selected) = self.params_list_state.selected() {
            let prev = if selected == 0 {
                self.params.len() - 1
            } else {
                selected - 1
            };
            self.params_list_state.select(Some(prev));
        }
    }

    fn select_next_param(&mut self) {
        if let Some(selected) = self.params_list_state.selected() {
            let next = (selected + 1) % self.params.len();
            self.params_list_state.select(Some(next));
        }
    }

    fn prev_option(&mut self) {
        if let Some(selected) = self.params_list_state.selected() {
            if let Some(param) = self.params.get(selected).cloned() {
                let new_param = param.toggle_prev();
                self.params[selected] = new_param;
            }
        }
    }

    fn next_option(&mut self) {
        if let Some(selected) = self.params_list_state.selected() {
            if let Some(param) = self.params.get(selected).cloned() {
                let new_param = param.toggle_next();
                self.params[selected] = new_param;
            }
        }
    }

    fn next_pane(&mut self) {
        self.current_pane = match self.current_pane {
            Pane::Info => Pane::Params,
            Pane::Params => Pane::Output,
            Pane::Output => Pane::Info,
        };
    }

    fn save(&mut self) {
        let mut args: Vec<&str> = Vec::new();
        for param in &self.params {
            match param {
                Param::DisableAudio(disable) => {
                    if *disable {
                        args.push("-an");
                    }
                }
                Param::AudioBitrate(bitrate) => {
                    if bitrate != &AudioBitrate::Auto {
                        args.push("-b:a");
                        args.push(bitrate.as_str());
                    }
                }
                Param::VideoBitrate(bitrate) => {
                    if bitrate != &VideoBitrate::Auto {
                        args.push("-b:v");
                        args.push(bitrate.as_str());
                    }
                }
            }
        }

        let output = Command::new("ffmpeg")
            .arg("-y")
            .arg("-hide_banner")
            .arg("-i")
            .arg(&self.input_file)
            .args(&args)
            .arg(format!("{}_out.mp4", self.input_file))
            .stderr(Stdio::piped())
            .output();
        if let Ok(output) = output
            && output.status.success()
        {
            self.output = String::from_utf8_lossy(&output.stderr).to_string();
        } else {
            self.output = "Failed to execute command".to_string();
        }
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
