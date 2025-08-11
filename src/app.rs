use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::ListState};

use crate::model::{AppEvent, AudioBitrate, Pane, Param, Parameter, VideoBitrate};

pub(crate) struct App {
    running: bool,
    event_sender: Sender<AppEvent>,
    pub(crate) current_pane: Pane,
    pub(crate) input_file: String,
    pub(crate) info_text: String,
    pub(crate) info_pane_current_line: u16,
    pub(crate) output: String,
    pub(crate) output_pane_current_line: u16,
    pub(crate) params: Vec<Param>,
    pub(crate) params_list_state: ListState,
}

impl App {
    pub fn new(tx: Sender<AppEvent>, input_file: String) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        App {
            running: false,
            event_sender: tx,
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
            output_pane_current_line: 0,
            params: vec![
                Param::DisableAudio(false),
                Param::AudioBitrate(AudioBitrate::Auto),
                Param::VideoBitrate(VideoBitrate::Auto),
            ],
            params_list_state: list_state,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, rx: Receiver<AppEvent>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            match rx.recv_timeout(Duration::from_secs(2)) {
                Ok(AppEvent::Input(key)) => self.on_key_event(key),
                Ok(AppEvent::SetOutput(output)) => self.set_output(output),
                Ok(AppEvent::AddOutput(output)) => self.add_output(output),
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn set_output(&mut self, output: String) {
        self.output = output;
        let count = self.output.lines().count() as u16;
        if count > 0 {
            self.output_pane_current_line = 0u16.saturating_add(count);
        } else {
            self.output_pane_current_line = 0;
        }
    }

    fn add_output(&mut self, output: String) {
        self.output.push_str(&output);
        self.output_pane_current_line = self
            .output_pane_current_line
            .saturating_add(output.lines().count() as u16)
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (self.current_pane, key.modifiers, key.code) {
            (_, _, KeyCode::Esc | KeyCode::Char('q'))
            | (_, KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, _, KeyCode::Tab) => self.next_pane(),
            (_, KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (Pane::Info, _, KeyCode::Down | KeyCode::Char('j')) => self.scroll_info_pane_down(),
            (Pane::Info, _, KeyCode::Up | KeyCode::Char('k')) => self.scroll_info_pane_up(),
            (Pane::Output, _, KeyCode::Down | KeyCode::Char('j')) => self.scroll_output_pane_down(),
            (Pane::Output, _, KeyCode::Up | KeyCode::Char('k')) => self.scroll_output_pane_up(),
            (Pane::Params, _, KeyCode::Down | KeyCode::Char('j')) => self.select_next_param(),
            (Pane::Params, _, KeyCode::Up | KeyCode::Char('k')) => self.select_prev_param(),
            (Pane::Params, _, KeyCode::Left | KeyCode::Char('h')) => self.prev_option(),
            (Pane::Params, _, KeyCode::Right | KeyCode::Char('l')) => self.next_option(),
            _ => {}
        }
    }

    fn scroll_info_pane_down(&mut self) {
        let count = self.info_text.lines().count() as u16;
        if self.info_pane_current_line < count - 1 {
            self.info_pane_current_line = self.info_pane_current_line.saturating_add(1);
        }
    }

    fn scroll_info_pane_up(&mut self) {
        if self.info_pane_current_line > 0 {
            self.info_pane_current_line = self.info_pane_current_line.saturating_sub(1);
        }
    }

    fn scroll_output_pane_down(&mut self) {
        let count = self.output.lines().count() as u16;
        if count > 0 && self.output_pane_current_line < count - 1 {
            self.output_pane_current_line = self.output_pane_current_line.saturating_add(1);
        }
    }

    fn scroll_output_pane_up(&mut self) {
        if self.output_pane_current_line > 0 {
            self.output_pane_current_line = self.output_pane_current_line.saturating_sub(1);
        }
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

        self.output_pane_current_line = 0;
        self.output = "Starting FFmpeg...\n".to_string();

        let input_file = self.input_file.clone();
        let tx = self.event_sender.clone();
        thread::spawn(move || {
            let mut child = match Command::new("ffmpeg")
                .arg("-y")
                .arg("-hide_banner")
                .arg("-i")
                .arg(&input_file)
                .args(&args)
                .arg(format!("{}_out.mp4", input_file))
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    let _ = tx.send(AppEvent::AddOutput(format!(
                        "Failed to start FFmpeg: {}\n",
                        e
                    )));
                    return;
                }
            };

            if let Some(stderr) = child.stderr.take() {
                let mut reader = BufReader::new(stderr);
                let mut buf = vec![0; 1024];
                while let Ok(read) = reader.read(&mut buf) {
                    if read == 0 {
                        break;
                    }
                    let line = String::from_utf8_lossy(&buf[..read])
                        .replace("\r\n", "\n")
                        .replace('\r', "\n");
                    let _ = tx.send(AppEvent::AddOutput(line));
                }
            }
            let msg = if let Ok(status) = child.wait()
                && status.success()
            {
                "FFmpeg finished successfully!\n\n"
            } else {
                "FFmpeg encountered an error.\n\n"
            };
            let _ = tx.send(AppEvent::AddOutput(msg.to_string()));
        });
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
