use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use app::App;
use crossterm::event::{Event, KeyEventKind};

use crate::{model::AppEvent, source::Source};

mod app;
mod info;
mod model;
mod params;
mod source;
mod ui;
mod visitors;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input", args[0]);
        std::process::exit(1);
    }
    let source = Source::new(args[1].clone());
    source.validate().map_err(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    })?;

    let ffprobe_info = match info::get_info(source.input.clone()) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error getting ffprobe info: {e}");
            std::process::exit(1);
        }
    };

    let terminal = ratatui::init();
    let (tx, rx) = mpsc::channel();
    let event_tx = tx.clone();
    thread::spawn(move || handle_crossterm_events(event_tx));

    let result = App::new(tx, ffprobe_info, source).run(terminal, rx);
    ratatui::restore();
    result
}

fn handle_crossterm_events(tx: Sender<AppEvent>) {
    loop {
        match crossterm::event::read() {
            Ok(Event::Key(key)) if key.kind == KeyEventKind::Press => {
                let _ = tx.send(AppEvent::Input(key));
            }
            Ok(Event::Resize(_, _)) => {
                let _ = tx.send(AppEvent::Redraw);
            }
            _ => {}
        }
    }
}
