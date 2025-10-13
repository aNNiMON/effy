use std::io::{BufReader, Read};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::{mem, thread};

use color_eyre::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::ListState};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

use crate::info::Info;
use crate::model::{AppEvent, Modal, Pane};
use crate::params::{Parameter, apply_visitor, create_params, recheck_params};
use crate::source::Source;
use crate::visitors::CommandBuilder;

pub(crate) struct App {
    running: bool,
    event_sender: Sender<AppEvent>,
    pub(crate) current_pane: Pane,
    pub(crate) source: Source,
    pub(crate) output_folder: String,
    pub(crate) output_filename: String,
    pub(crate) output_fileext: String,
    pub(crate) info_text: String,
    pub(crate) info_pane_current_line: u16,
    pub(crate) output: String,
    pub(crate) output_pane_current_line: u16,
    pub(crate) params: Vec<Parameter>,
    pub(crate) params_list_state: ListState,
    pub(crate) modal: Option<Modal>,
    save_ongoing: bool,
}

impl App {
    pub fn new(tx: Sender<AppEvent>, info: Info, source: Source) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let folder = source.input_folder();
        let (filename, fileext) = source.input_name_and_ext();
        Self {
            running: false,
            event_sender: tx,
            current_pane: Pane::Params,
            source,
            output_folder: folder,
            output_filename: format!("{filename}_out"),
            output_fileext: fileext.to_string(),
            info_text: info.format(),
            info_pane_current_line: 0,
            output: "".to_string(),
            output_pane_current_line: 0,
            params: create_params(&info),
            params_list_state: list_state,
            modal: None,
            save_ongoing: false,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal, rx: Receiver<AppEvent>) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            match rx.recv() {
                Ok(AppEvent::Input(key)) => self.on_key_event(key),
                Ok(AppEvent::AddOutput(output)) => self.add_output(output),
                Ok(AppEvent::SaveCompleted(success)) => self.on_save_complete(success),
                Ok(AppEvent::Redraw) => {}
                Err(_) => {}
            }
        }
        Ok(())
    }

    fn add_output(&mut self, output: String) {
        self.output.push_str(&output);
        self.output_pane_current_line = 0;
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        if let Some(modal) = &self.modal {
            modal.render(frame);
        }
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if self.modal.is_some() {
            self.handle_modal_input(key);
            return;
        }
        match (self.current_pane, key.modifiers, key.code) {
            (_, _, KeyCode::Esc | KeyCode::Char('q'))
            | (_, KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, _, KeyCode::Tab) => self.next_pane(),
            (_, KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (_, _, KeyCode::Char('s')) => {
                self.modal = Some(Modal::SaveFileAs(
                    Input::default().with_value(self.output_filename.clone()),
                ))
            }
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

    fn handle_modal_input(&mut self, key: KeyEvent) {
        match &mut self.modal {
            Some(Modal::SaveFileAs(input)) => {
                if key.code == KeyCode::Esc {
                    self.modal = None;
                } else if key.code == KeyCode::Enter {
                    let filename = input.value();
                    let valid = !filename.trim().is_empty() && !Path::new(filename).exists();
                    if valid {
                        self.output_filename = input.value().to_string();
                        self.save();
                    }
                } else {
                    input.handle_event(&crossterm::event::Event::Key(key));
                }
            }
            None => {}
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
        if count > 0 {
            self.output_pane_current_line = self.output_pane_current_line.saturating_sub(1);
        }
    }

    fn scroll_output_pane_up(&mut self) {
        let count = self.output.lines().count() as u16;
        if self.output_pane_current_line < count {
            self.output_pane_current_line =
                self.output_pane_current_line.saturating_add(1).min(count);
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
        if let Some(selected) = self.params_list_state.selected()
            && let Some(mut param) = self.params.get_mut(selected).map(mem::take)
        {
            param.toggle_prev();
            recheck_params(&mut self.params, &param);
            self.params[selected] = param;
        }
    }

    fn next_option(&mut self) {
        if let Some(selected) = self.params_list_state.selected()
            && let Some(mut param) = self.params.get_mut(selected).map(mem::take)
        {
            param.toggle_next();
            recheck_params(&mut self.params, &param);
            self.params[selected] = param;
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
        if self.save_ongoing {
            return;
        }
        self.modal = None;
        self.save_ongoing = true;

        let mut command_builder = CommandBuilder::new();
        apply_visitor(&mut command_builder, &self.params);
        self.output_pane_current_line = 0;
        self.output = "Starting FFmpeg...\n".to_string();

        let input = self.source.input.clone();
        let output_file = format!(
            "{}/{}.{}",
            self.output_folder, self.output_filename, self.output_fileext
        );
        let tx = self.event_sender.clone();
        thread::spawn(move || {
            let mut child = match Command::new("ffmpeg")
                .arg("-y")
                .arg("-hide_banner")
                .args(command_builder.build_pre_input_args())
                .arg("-i")
                .arg(&input)
                .args(command_builder.build_args())
                .arg(&output_file)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
            {
                Ok(child) => child,
                Err(e) => {
                    let _ = tx.send(AppEvent::AddOutput(format!(
                        "Failed to start FFmpeg: {e}\n"
                    )));
                    let _ = tx.send(AppEvent::SaveCompleted(false));
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
            let result = matches!(child.wait(), Ok(status) if status.success());
            let _ = tx.send(AppEvent::SaveCompleted(result));
        });
    }

    fn on_save_complete(&mut self, success: bool) {
        let msg = if success {
            "FFmpeg finished successfully!\n\n"
        } else {
            "FFmpeg encountered an error.\n\n"
        };
        self.add_output(msg.to_string());
        self.save_ongoing = false;
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
