use crossterm::event::*;
use std::io::Result;

#[derive(Debug, PartialEq)]
pub enum Action {
    Quit,
    Insert,
    Restart,
    None,
    Delete,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    NewLine,
    Char(char),
}

impl Into<char> for Token {
    fn into(self) -> char {
        match self {
            Token::NewLine => '\n',
            Token::Char(c) => c,
        }
    }
}

#[derive(Debug)]
pub struct EventResult {
    pub action: Action,
    pub token: Option<Token>,
}

impl Default for EventResult {
    fn default() -> Self {
        Self {
            action: Action::None,
            token: None,
        }
    }
}

fn handle_key_event(key_event: KeyEvent) -> EventResult {
    if key_event.kind != KeyEventKind::Press {
        return EventResult {
            action: Action::None,
            token: None,
        };
    }

    let mut event_result = EventResult::default();

    match key_event.code {
        KeyCode::Char('q') => {
            if key_event.modifiers.contains(KeyModifiers::ALT) {
                event_result.action = Action::Quit;
            } else {
                event_result.action = Action::Insert;
                event_result.token = Some(Token::Char('q'));
            }
        }
        KeyCode::Char('c') => {
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                event_result.action = Action::Quit;
            } else {
                event_result.action = Action::Insert;
                event_result.token = Some(Token::Char('q'));
            }
        }
        KeyCode::Char('r') => {
            if key_event.modifiers.contains(KeyModifiers::ALT) {
                event_result.action = Action::Restart;
            } else {
                event_result.action = Action::Insert;
                event_result.token = Some(Token::Char('r'));
            }
        }
        KeyCode::Enter => {
            event_result.action = Action::Insert;
            event_result.token = Some(Token::NewLine);
        }
        KeyCode::Backspace => event_result.action = Action::Delete,
        code => {
            if let Some(char) = code.as_char() {
                event_result.action = Action::Insert;
                event_result.token = Some(Token::Char(char));
            } else {
                event_result.action = Action::None;
            }
        }
    };

    event_result
}

pub fn handle_events() -> Result<EventResult> {
    if !poll(std::time::Duration::from_millis(16))? {
        return Ok(EventResult {
            action: Action::None,
            token: None,
        });
    }

    let event = read()?;

    let key_event = match event {
        Event::Key(key_event) => key_event,
        _ => {
            return Ok(EventResult {
                action: Action::None,
                token: None,
            });
        }
    };

    let event_result = handle_key_event(key_event);

    Ok(event_result)
}
