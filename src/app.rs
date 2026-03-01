use std::error::Error;
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{ChildStdin, Command, Stdio};
use std::sync::mpsc::{Receiver, Sender};
use std::{mem, thread};

use arboard::Clipboard;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{DefaultTerminal, widgets::ListState};
use tracing::debug;

use crate::info::Info;
use crate::model::{AppEvent, Pane};
use crate::params::{
    Parameter, ParameterData, Trim, apply_visitor, create_params, get_output_format,
    recheck_params, save_preset,
};
use crate::source::Source;
use crate::ui::state::{InfoPaneState, OutputPaneState};
use crate::ui::{
    AlertKind, AlertModal, CustomSelectModal, HelpModal, ModalResult, SaveAsFileModal, Theme,
    TrimModal, UiModal,
};
use crate::visitors::CommandBuilder;

pub(crate) struct App<'a> {
    // App state
    running: bool,
    event_sender: Sender<AppEvent>,
    clipboard: Result<Clipboard, ()>,
    // UI
    pub current_pane: Pane,
    pub active_out_pane: Pane,
    modal: Option<Box<dyn UiModal>>,
    pub theme: Theme,
    // Params
    pub params: Vec<Parameter>,
    pub params_list_state: ListState,
    // Input
    pub original_filename: Box<str>,
    pub source: Source,
    pub info_state: InfoPaneState<'a>,
    // Output
    pub out_state: OutputPaneState,
    output_folder: String,
    output_filename: String,
    output_fileext: String,
    pub save_ongoing: bool,
    render_stdin: Option<ChildStdin>,
}

impl App<'_> {
    pub fn new(tx: Sender<AppEvent>, info: &Info, source: Source, preset: Option<&str>) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let folder = source.input_folder();
        let theme = Theme::new();
        let info_state = InfoPaneState::new(info.format(&theme));
        let (filename, fileext) = source.input_name_and_ext(info);
        let original_filename = if source.is_url() {
            "".into()
        } else {
            filename.clone().into()
        };
        Self {
            running: false,
            event_sender: tx,
            clipboard: Clipboard::new().map_err(|_| ()),
            // UI
            current_pane: Pane::Params,
            active_out_pane: Pane::Info,
            modal: None,
            theme,
            // Params
            params: create_params(info, preset, fileext.as_str()),
            params_list_state: list_state,
            // Info
            original_filename,
            source,
            info_state,
            // Output
            out_state: OutputPaneState::new(String::new()),
            output_folder: folder,
            output_filename: format!("{filename}_out"),
            output_fileext: fileext.clone(),
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
            terminal.draw(|frame| {
                frame.render_widget(&mut self, frame.area());
                if let Some(modal) = &self.modal {
                    modal.render(frame, &self.theme);
                }
            })?;
            match rx.recv() {
                Ok(AppEvent::Input(key)) => self.on_key_event(key),
                Ok(AppEvent::AddOutput(output)) => self.out_state.add_output(&output),
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

    pub fn run_cli(&mut self) {
        let args = self.build_ffmpeg_command(false);
        println!("Starting FFmpeg\nCommand: ffmpeg {}", args.join(" "));
        Command::new("ffmpeg")
            .args(args)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .expect("Failed to start FFmpeg");
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
            (_, _, KeyCode::Char('i')) => self.select_info_pane(),
            (_, _, KeyCode::Char('o')) => self.select_output_pane(),
            (_, _, KeyCode::F(1) | KeyCode::Char('?')) => self.help(),
            (_, KeyModifiers::CONTROL, KeyCode::Char('s')) => self.save(),
            (_, _, KeyCode::Char('s')) => {
                let output_ext = get_output_format(&self.params)
                    .map_or(&self.output_fileext, |option| &option.value);
                self.modal = Some(Box::new(SaveAsFileModal::new(
                    &self.original_filename,
                    &self.output_folder,
                    &self.output_filename,
                    output_ext,
                )));
            }
            (Pane::Info, _, KeyCode::Down | KeyCode::Char('j')) => self.info_state.scroll_down(),
            (Pane::Info, _, KeyCode::Up | KeyCode::Char('k')) => self.info_state.scroll_up(),
            (Pane::Output, _, KeyCode::Down | KeyCode::Char('j')) => self.out_state.scroll_down(),
            (Pane::Output, _, KeyCode::Up | KeyCode::Char('k')) => self.out_state.scroll_up(),
            (Pane::Params, _, KeyCode::Down | KeyCode::Char('j')) => self.select_next_param(),
            (Pane::Params, _, KeyCode::Up | KeyCode::Char('k')) => self.select_prev_param(),
            (Pane::Params, _, KeyCode::Left | KeyCode::Char('h')) => self.prev_option(),
            (Pane::Params, _, KeyCode::Right | KeyCode::Char('l')) => self.next_option(),
            (Pane::Params, _, KeyCode::Enter) => self.open_param_modal(),
            (Pane::Params, _, KeyCode::Char('p')) => self.copy_preset(),
            (Pane::Params, _, KeyCode::Char('y')) => self.copy_command(),
            _ => {}
        }
    }

    fn copy_preset(&mut self) {
        let (kind, msg) = match self.clipboard.as_mut().map(|ctx| {
            let preset = save_preset(&mut self.params);
            ctx.set_text(preset)
        }) {
            Ok(_) => (AlertKind::Info, "Preset has been copied to clipboard"),
            Err(_) => (AlertKind::Error, "Failed to copy the preset to clipboard"),
        };
        self.modal = Some(Box::new(AlertModal::new(kind, msg)));
    }

    fn copy_command(&mut self) {
        let args = self.build_ffmpeg_command(false);
        let (kind, msg) = match self
            .clipboard
            .as_mut()
            .map(|ctx| ctx.set_text(format!("ffmpeg {}", args.join(" "))))
        {
            Ok(_) => (AlertKind::Info, "Command has been copied to clipboard"),
            Err(_) => (AlertKind::Error, "Failed to copy the command to clipboard"),
        };
        self.modal = Some(Box::new(AlertModal::new(kind, msg)));
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
        if self.current_pane == Pane::Params {
            self.active_out_pane = Pane::Output;
        } else {
            self.active_out_pane = Pane::Info;
        }
        self.current_pane = match self.current_pane {
            Pane::Info => Pane::Params,
            Pane::Params => Pane::Output,
            Pane::Output => Pane::Info,
        };
    }

    fn next_pane(&mut self) {
        if self.current_pane == Pane::Params {
            self.active_out_pane = Pane::Info
        } else {
            self.active_out_pane = Pane::Output;
        }
        self.current_pane = match self.current_pane {
            Pane::Info => Pane::Output,
            Pane::Params => Pane::Info,
            Pane::Output => Pane::Params,
        };
    }

    fn select_info_pane(&mut self) {
        self.active_out_pane = Pane::Info;
        self.current_pane = Pane::Info;
    }

    fn select_output_pane(&mut self) {
        self.active_out_pane = Pane::Output;
        self.current_pane = Pane::Output;
    }

    fn help(&mut self) {
        self.modal = Some(Box::new(HelpModal::new(&self.theme)));
    }

    fn save(&mut self) {
        if self.save_ongoing {
            return;
        }
        self.modal = None;
        self.save_ongoing = true;
        self.active_out_pane = Pane::Output;

        let args = self.build_ffmpeg_command(true);
        debug!(?args, "Starting FFmpeg");
        self.out_state.set_output("Starting FFmpeg...\n");

        let tx = self.event_sender.clone();
        thread::spawn(move || {
            let mut child = match Command::new("ffmpeg")
                .args(&args)
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
        self.out_state.add_output(msg);
        self.save_ongoing = false;
        self.render_stdin = None;
    }

    fn quit(&mut self) {
        if self.save_ongoing
            && let Some(mut stdin) = self.render_stdin.take()
        {
            let _ = stdin.write_all(b"q");
            self.out_state.add_output("Stopping...\n");
        } else {
            self.running = false;
        }
    }

    fn build_ffmpeg_command(&mut self, overwrite: bool) -> Vec<String> {
        let mut command_builder = CommandBuilder::default();
        apply_visitor(&mut command_builder, &mut self.params);
        let input = self.source.input.clone();
        let mut path = PathBuf::new()
            .join(&*self.output_folder)
            .join(&*self.output_filename);
        path.add_extension(&*command_builder.ext);
        let output_file = path.display().to_string();

        let mut args: Vec<String> = Vec::new();
        if overwrite {
            args.push("-y".into());
        }
        args.push("-hide_banner".into());
        args.extend(command_builder.build_pre_input_args().iter().cloned());
        args.push("-i".into());
        args.push(input);
        args.extend(command_builder.build_args());
        args.push(output_file);
        args
    }
}
