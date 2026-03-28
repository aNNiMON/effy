use std::{
    error::Error,
    process,
    sync::mpsc::{self, Sender},
    thread,
};

use app::App;
use clap::Parser;
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

#[derive(Debug, Parser)]
#[command(
    name = "effy",
    version,
    about = "A small and friendly terminal FFmpeg helper that simplifies common tasks"
)]
struct Cli {
    /// Specify parameter values
    #[arg(short, long)]
    preset: Option<String>,

    /// Apply preset immediately without UI
    #[arg(long, requires = "preset")]
    apply: bool,

    /// Media file or URL
    input: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let _guard = logging::init_tracing();

    let cli = Cli::parse();
    let source = Source::new(cli.input);
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

    if cli.apply {
        let (tx, _) = mpsc::channel();
        App::new(tx, &ffprobe_info, source, cli.preset.as_deref()).run_cli();
        process::exit(0);
    }

    ratatui::run(|terminal| {
        let (tx, rx) = mpsc::channel();
        let event_tx = tx.clone();
        thread::spawn(move || handle_crossterm_events(&event_tx));
        App::new(tx, &ffprobe_info, source, cli.preset.as_deref()).run(terminal, &rx)
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
