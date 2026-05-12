mod app;
mod args;
mod input;
mod ui;

use app::App;
use args::parseArgs;
use input::handle_events;
use ui::UI;

use std::fs;
use std::io::{self, Result, Stdout};

use crossterm::{
    event::*,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use ratatui::{DefaultTerminal, Terminal, prelude::CrosstermBackend};

fn main() -> Result<()> {
    let options = parseArgs();

    let text = if let Ok(file) = fs::read_to_string("") {
        file.chars().take(30).collect()
    } else {
        DEFAULT_TEXT.to_string()
    };
    return Ok(());

    let mut terminal = setup_terminal()?;
    let mut app = App::new(text.as_str());
    run(&mut terminal, &mut app, text.as_str())?;
    teardown_terminal()?;
    Ok(())
}

fn run(terminal: &mut DefaultTerminal, app: &mut App, text: &str) -> std::io::Result<()> {
    let terminal_size = terminal.size()?;
    let ui = UI::new(terminal_size.width, terminal_size.height);

    loop {
        if app.should_quit {
            return Ok(());
        };

        let event_result = handle_events()?;
        app.update(&event_result);

        terminal.draw(|f| ui.update(f, app, text))?;
    }
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        SetTitle("Typocalypse")
    )?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn teardown_terminal() -> Result<()> {
    let mut stdout = io::stdout();
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
