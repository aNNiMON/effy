use std::error::Error;
use std::io::{BufReader, Read, Write};
use std::process::{ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::{mem, thread};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, Frame, widgets::ListState};

use crate::info::Info;
use crate::model::{AppEvent, Pane};
use crate::params::{Parameter, ParameterData, Trim, apply_visitor, create_params, recheck_params};
use crate::source::Source;
use crate::ui::{CustomSelectModal, ModalResult, SaveAsFileModal, TrimModal, UiModal};
use crate::visitors::CommandBuilder;

pub(crate) struct App {
    running: bool,
    event_sender: Sender<AppEvent>,
    pub(crate) current_pane: Pane,
    pub(crate) source: Source,
    output_folder: String,
    output_filename: String,
    output_fileext: String,
    pub(crate) info_text: String,
    pub(crate) info_pane_current_line: u16,
    pub(crate) output: String,
    pub(crate) output_pane_current_line: u16,
    pub(crate) params: Vec<Parameter>,
    pub(crate) params_list_state: ListState,
    modal: Option<Box<dyn UiModal>>,
    pub(crate) save_ongoing: bool,
    render_stdin: Option<ChildStdin>,
}

impl App {
    pub fn new(tx: Sender<AppEvent>, info: &Info, source: Source) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let folder = source.input_folder();
        let (filename, fileext) = source.input_name_and_ext(info);
        Self {
            running: false,
            event_sender: tx,
            current_pane: Pane::Params,
            source,
            output_folder: folder,
            output_filename: format!("{filename}_out"),
            output_fileext: fileext.clone(),
            info_text: info.format(),
            info_pane_current_line: 0,
            output: String::new(),
            output_pane_current_line: 0,
            params: create_params(info, fileext.as_str()),
            params_list_state: list_state,
            modal: None,
            save_ongoing: false,
            render_stdin: None,
        }
    }

    pub fn run(
        mut self,
        terminal: &mut DefaultTerminal,
        rx: &Receiver<AppEvent>,
    ) -> Result<(), Box<dyn Error>> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            match rx.recv() {
                Ok(AppEvent::Input(key)) => self.on_key_event(key),
                Ok(AppEvent::AddOutput(output)) => self.add_output(&output),
                Ok(AppEvent::SaveCompleted(success)) => self.on_save_complete(success),
                Ok(AppEvent::OpenTrimModal(data)) => {
                    self.modal = Some(Box::new(TrimModal::from(data)));
                }
                Ok(AppEvent::OpenCustomSelectModal(data)) => {
                    self.modal = Some(Box::new(CustomSelectModal::from(data)));
                }
                Ok(AppEvent::RenderStarted(stdin)) => self.render_stdin = Some(stdin),
                Ok(AppEvent::Redraw) | Err(_) => {}
            }
        }
        Ok(())
    }

    fn add_output(&mut self, output: &str) {
        self.output.push_str(output);
        self.output_pane_current_line = 0;
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
        if let Some(modal) = &self.modal {
            modal.render(frame);
        }
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        if let Some(modal) = &mut self.modal {
            match modal.handle_key(key) {
                ModalResult::Close => self.modal = None,
                ModalResult::Filename(filename) => {
                    self.output_filename.clone_from(&filename);
                    self.save();
                }
                ModalResult::Trim => {
                    if let Some(param) = self.params.iter_mut().find(|p| p.id == Trim::ID)
                        && let ParameterData::Trim(data) = &mut param.data
                        && let Some(trim) = modal.downcast_ref::<TrimModal>()
                    {
                        *data = trim.into();
                    }
                    self.modal = None;
                }
                ModalResult::CustomSelect(value) => {
                    if let Some(selected) = self.params_list_state.selected()
                        && let Some(param) = self.params.get_mut(selected)
                        && let ParameterData::CustomSelect {
                            value: param_value, ..
                        } = &mut param.data
                    {
                        param_value.clone_from(&value);
                    }
                    self.modal = None;
                }
                ModalResult::None => {}
            }
            return;
        }
        match (self.current_pane, key.modifiers, key.code) {
            (_, _, KeyCode::Esc | KeyCode::Char('q'))
            | (_, KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_, _, KeyCode::BackTab) => self.prev_pane(),
            (_, _, KeyCode::Tab) => self.next_pane(),
            (_, KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (_, _, KeyCode::Char('s')) => {
                self.modal = Some(Box::new(SaveAsFileModal::new(
                    &self.output_folder,
                    &self.output_filename,
                    &self.output_fileext,
                )));
            }
            (Pane::Info, _, KeyCode::Down | KeyCode::Char('j')) => self.scroll_info_pane_down(),
            (Pane::Info, _, KeyCode::Up | KeyCode::Char('k')) => self.scroll_info_pane_up(),
            (Pane::Output, _, KeyCode::Down | KeyCode::Char('j')) => self.scroll_output_pane_down(),
            (Pane::Output, _, KeyCode::Up | KeyCode::Char('k')) => self.scroll_output_pane_up(),
            (Pane::Params, _, KeyCode::Down | KeyCode::Char('j')) => self.select_next_param(),
            (Pane::Params, _, KeyCode::Up | KeyCode::Char('k')) => self.select_prev_param(),
            (Pane::Params, _, KeyCode::Left | KeyCode::Char('h')) => self.prev_option(),
            (Pane::Params, _, KeyCode::Right | KeyCode::Char('l')) => self.next_option(),
            (Pane::Params, _, KeyCode::Enter) => self.open_param_modal(),
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
            param.toggle_prev(&self.event_sender);
            self.params[selected] = param;
            recheck_params(&mut self.params);
        }
    }

    fn next_option(&mut self) {
        if let Some(selected) = self.params_list_state.selected()
            && let Some(mut param) = self.params.get_mut(selected).map(mem::take)
        {
            param.toggle_next(&self.event_sender);
            self.params[selected] = param;
            recheck_params(&mut self.params);
        }
    }

    fn open_param_modal(&mut self) {
        if let Some(selected) = self.params_list_state.selected()
            && let Some(param) = self.params.get(selected)
        {
            param.open_modal(&self.event_sender);
        }
    }

    fn prev_pane(&mut self) {
        self.current_pane = match self.current_pane {
            Pane::Info => Pane::Output,
            Pane::Params => Pane::Info,
            Pane::Output => Pane::Params,
        };
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

        let mut command_builder = CommandBuilder::default();
        apply_visitor(&mut command_builder, &self.params);
        self.output_pane_current_line = 0;
        "Starting FFmpeg...\n".clone_into(&mut self.output);

        let input = self.source.input.clone();
        let output_file = format!(
            "{}/{}.{}",
            self.output_folder, self.output_filename, command_builder.ext
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
                .stdin(Stdio::piped())
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

            if let Some(stdin) = child.stdin.take() {
                let _ = tx.send(AppEvent::RenderStarted(stdin));
            }

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
        self.add_output(msg);
        self.save_ongoing = false;
        self.render_stdin = None;
    }

    fn quit(&mut self) {
        if self.save_ongoing
            && let Some(mut stdin) = self.render_stdin.take()
        {
            let _ = stdin.write_all(b"q");
            self.add_output("Stopping...\n");
        } else {
            self.running = false;
        }
    }
}
