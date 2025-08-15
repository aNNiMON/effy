use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use app::App;
use crossterm::event::{Event, KeyEventKind};

use crate::model::AppEvent;

mod app;
mod info;
mod model;
mod ui;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} input", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    if !std::fs::metadata(input).is_ok() {
        eprintln!("Error: File '{}' does not exist", input);
        std::process::exit(1);
    }
    let ffprobe_output = match info::get_info(input.clone()) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error getting ffprobe info: {}", e);
            std::process::exit(1);
        }
    };

    let terminal = ratatui::init();
    let (tx, rx) = mpsc::channel();
    let event_tx = tx.clone();
    thread::spawn(move || handle_crossterm_events(event_tx));

    let result = App::new(tx, ffprobe_output, input.clone()).run(terminal, rx);
    ratatui::restore();
    result
}

fn handle_crossterm_events(tx: Sender<AppEvent>) {
    loop {
        if let Ok(Event::Key(key)) = crossterm::event::read()
            && key.kind == KeyEventKind::Press
        {
            let _ = tx.send(AppEvent::Input(key));
        }
    }
}
