mod app;
mod args;
mod input;
mod terminal;
mod ui;

use app::App;
use args::parse_args;
use input::handle_events;

use std::fs;
use std::io::Result;

use crate::terminal::{TerminalGuard, run, setup_terminal};

fn main() -> Result<()> {
    let _guard = TerminalGuard;
    let options = parse_args();

    let text = match &options.path {
        Some(path) => match fs::read_to_string(path) {
            Ok(file) => file.chars().take(options.limit).collect(),
            Err(_) => {
                eprintln!("error: failed to open file {}", path);
                std::process::exit(2);
            }
        },
        None => options.text,
    };

    let trimmed = text.trim();

    let mut terminal = setup_terminal()?;
    let mut app = App::new(trimmed);
    run(&mut terminal, &mut app, trimmed)?;

    Ok(())
}
