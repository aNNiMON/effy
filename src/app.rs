use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Info,
    Params,
    Config,
}

pub struct App {
    running: bool,
    pub current_pane: Pane,
    pub info_text: String,
    pub info_pane_current_line: u16,
}

impl App {
    pub fn new() -> Self {
        App {
            running: false,
            current_pane: Pane::Info,
            info_text: "Input file: abc.mp4\n\
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
                 Audio Channels: 2"
                .to_string(),
            info_pane_current_line: 0,
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
            (Pane::Info, _, KeyCode::Down | KeyCode::Char('j')) => self.scroll_info_pane_down(),
            (Pane::Info, _, KeyCode::Up | KeyCode::Char('k')) => self.scroll_info_pane_up(),
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

    fn next_pane(&mut self) {
        self.current_pane = match self.current_pane {
            Pane::Info => Pane::Params,
            Pane::Params => Pane::Config,
            Pane::Config => Pane::Info,
        };
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
