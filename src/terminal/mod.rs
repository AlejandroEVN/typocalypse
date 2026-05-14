use crate::handle_events;
use crate::{app::App, ui::UI};
use std::io::{self, Result, Stdout};

use crossterm::{
    event::*,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, SetTitle},
};
use ratatui::{DefaultTerminal, Terminal, prelude::CrosstermBackend};

pub fn run(terminal: &mut DefaultTerminal, app: &mut App, text: &str) -> std::io::Result<()> {
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

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
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

pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let mut stdout = std::io::stdout();
        let _ = crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture);
    }
}
