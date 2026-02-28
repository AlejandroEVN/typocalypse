mod app;
mod input;
mod ui;

use app::App;
use input::handle_events;
use ui::UI;

use std::io::{self, Result, Stdout};

use crossterm::{
    event::*,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use ratatui::{DefaultTerminal, Terminal, prelude::CrosstermBackend};

pub const PARAGRAPH: &str = "Lorem ipsum dolor sit amet";

pub const HELP: &str = "Something something";

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let mut app = App::new();
    run(&mut terminal, &mut app)?;
    teardown_terminal()?;
    Ok(())
}

fn run(terminal: &mut DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    let terminal_size = terminal.size()?;
    let ui = UI::new(terminal_size.width, terminal_size.height);

    loop {
        if app.should_quit {
            return Ok(());
        };

        let event_result = handle_events()?;
        app.update(&event_result);

        terminal.draw(|f| ui.update(f, app))?;
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
