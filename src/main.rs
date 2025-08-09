use app::App;

mod app;
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

    let terminal = ratatui::init();
    let result = App::new(input.clone()).run(terminal);
    ratatui::restore();
    result
}
