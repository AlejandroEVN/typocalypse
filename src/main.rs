mod app;
mod args;
mod db;
mod input;
mod terminal;
mod ui;

use crate::app::App;
use crate::args::parse_args;
use crate::db::DB;
use crate::input::handle_events;
use crate::terminal::{TerminalGuard, run, setup_terminal};

use directories::ProjectDirs;
use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let dirs = init();
    let db = DB::new(dirs.data_dir());
    let _guard = TerminalGuard;

    let options = parse_args();

    if options.should_reset {
        let _ = db.reset_results();
    }

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
    let mut app = App::new(db, trimmed);
    run(&mut terminal, &mut app, trimmed)?;

    Ok(())
}

fn init() -> ProjectDirs {
    let project_dirs = ProjectDirs::from("", "", "typocalypse")
        .expect("error: could not determine project directories");

    fs::create_dir_all(project_dirs.data_dir()).expect("error: creating .local data folder");

    project_dirs
}
