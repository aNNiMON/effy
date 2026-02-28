use std::{
    error::Error,
    process,
    sync::mpsc::{self, Sender},
    thread,
};

use app::App;
use crossterm::event::{Event, KeyEventKind};

use crate::{model::AppEvent, source::Source};

mod app;
mod info;
mod logging;
mod model;
mod params;
mod source;
mod ui;
mod visitors;

fn main() -> Result<(), Box<dyn Error>> {
    let _guard = logging::init_tracing();

    let mut args: Vec<String> = std::env::args().skip(1).rev().collect();
    let mut preset = None;
    let mut input = None;
    let mut apply = false;
    while let Some(arg) = args.pop() {
        if arg == "--preset" {
            if args.is_empty() {
                eprintln!("Error: --preset requires an argument");
                process::exit(1);
            }
            preset = args.pop();
        } else if arg == "--apply" {
            apply = true;
        } else {
            input = Some(arg);
        }
    }

    if input.is_none() {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("effy v{VERSION}");
        println!("Usage: effy [--preset <preset>] [--apply] <input>");
        println!("\nArguments:");
        println!("  <input>    Media file or URL");
        println!("\nOptions:");
        println!("  --preset <preset>  Specify parameter values");
        println!("  --apply            Apply preset immediately without UI");
        process::exit(1);
    }
    let source = Source::new(input.unwrap());
    source.validate().map_err(|e| {
        eprintln!("Error: {e}");
        process::exit(1);
    })?;

    let ffprobe_info = match info::get_info(&source.input) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error getting ffprobe info: {e}");
            process::exit(1);
        }
    };

    if apply {
        if preset.is_none() {
            eprintln!("Preset is required when --apply is specified");
            process::exit(1);
        } else {
            let (tx, _) = mpsc::channel();
            App::new(tx, &ffprobe_info, source, preset.as_deref()).run_cli();
            process::exit(0);
        }
    }

    ratatui::run(|terminal| {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || handle_crossterm_events(&event_tx));
        App::new(tx, &ffprobe_info, source, preset.as_deref()).run(terminal, &rx)
    })
}

fn handle_crossterm_events(tx: &Sender<AppEvent>) -> ! {
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
