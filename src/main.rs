use std::{
    io::{self, Result, Stdout},
    time::Instant,
};

use crossterm::{
    event::*,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    DefaultTerminal, Frame, Terminal,
    layout::{Constraint, Layout, Rect},
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

const PARAGRAPH: &str = "Lorem ipsum dolor sit amet";

const HELP: &str = "Something something";

#[derive(Debug, PartialEq)]
enum Action {
    Quit,
    Insert,
    Restart,
    None,
    Delete,
}

#[derive(Debug, Clone, Copy)]
enum Token {
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
struct EventResult {
    action: Action,
    token: Option<Token>,
}

impl Default for EventResult {
    fn default() -> Self {
        Self {
            action: Action::None,
            token: None,
        }
    }
}

#[derive(Default, Debug)]
struct TypingStats {
    typed_raw: u16,
    correct: u16,
    incorrect: u16,
    typed: u16,
    misstyped: u16,
    extra: u16,
}

#[derive(Default)]
struct Stats {
    typing: TypingStats,
    accuracy: f32,
    wpm: f32,
    time_in_seconds: u64,
    started: Option<Instant>,
    finished: Option<Instant>,
}

struct AppLayout {
    top: Rect,
    bottom: Rect,
}

struct App {
    text: String,
    typed_text: String,
    should_quit: bool,
    stats: Stats,
    show_result: bool,
    layout: AppLayout,
}

impl App {
    fn new(terminal: &DefaultTerminal) -> Result<App> {
        let terminal_size = terminal.size()?;
        let area = Rect::new(0, 0, terminal_size.width, terminal_size.height);
        let (areas, _) = Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)])
            .split_with_spacers(area);
        let app_layout = AppLayout {
            top: areas[0],
            bottom: areas[1],
        };

        Ok(App {
            text: PARAGRAPH.to_string(),
            typed_text: String::new(),
            should_quit: false,
            stats: Stats::default(),
            show_result: false,
            layout: app_layout,
        })
    }

    fn run(mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            if self.should_quit {
                return Ok(());
            };

            self.handle_events()?;

            terminal.draw(|f| self.render(f))?;
        }
    }

    fn reset(&mut self) {
        self.typed_text.clear();
        self.should_quit = false;
        self.stats = Stats::default();
        self.show_result = false;
    }

    fn render_footer(&self, frame: &mut Frame) {
        let mut footer_content = format!("{} - Current: {}", HELP, 0);

        if self.show_result {
            let started = self.stats.started.unwrap();
            let finished = self.stats.finished.unwrap();
            footer_content = format!(
                "{} - Total: {} secs",
                HELP,
                finished.checked_duration_since(started).unwrap().as_secs()
            );
        } else if let Some(started) = self.stats.started {
            footer_content = format!("{} - Current: {} secs", HELP, started.elapsed().as_secs());
        }

        let footer = Paragraph::new(footer_content);

        frame.render_widget(footer, self.layout.bottom);
    }

    fn render(&mut self, frame: &mut Frame) {
        if self.show_result {
            return self.render_result(frame);
        }

        let mut spans = Vec::<Span>::new();
        let mut typed_iter = self.typed_text.chars();
        let mut lines = Vec::<Line>::new();

        for expected in self.text.chars() {
            match expected {
                '\n' => {
                    lines.push(Line::from(spans));
                    spans = Vec::new();
                }
                _ => {
                    let span = match typed_iter.next() {
                        Some(actual) => {
                            if expected == actual {
                                Span::styled(
                                    expected.to_string(),
                                    Style::default().fg(Color::Green),
                                )
                            } else {
                                let is_whitespace = expected == ' ';

                                if is_whitespace {
                                    Span::styled(
                                        ' '.to_string(),
                                        Style::default().bg(Color::Red).fg(Color::White),
                                    )
                                } else {
                                    Span::styled(
                                        expected.to_string(),
                                        Style::default().fg(Color::Red),
                                    )
                                }
                            }
                        }
                        None => Span::raw(expected.to_string()),
                    };
                    spans.push(span);
                }
            }
        }

        if !spans.is_empty() {
            lines.push(Line::from(spans));
        }

        let text = Text::from(lines);
        let text_paragraph = Paragraph::new(text)
            .centered()
            .block(Block::default().padding(Padding::new(16, 16, 16, 16)))
            .style(Style::new().add_modifier(Modifier::BOLD))
            .wrap(Wrap { trim: true });

        frame.render_widget(text_paragraph, self.layout.top);
        self.render_footer(frame);
    }

    fn render_result(&mut self, frame: &mut Frame) {
        self.calculate_result();

        let text = format!(
            "WPM: {} | {}/{}/{} | Accuracy: {} | Seconds: {} \n {:?}",
            self.stats.wpm,
            self.stats.typing.typed,
            self.stats.typing.misstyped,
            self.stats.typing.extra,
            self.stats.accuracy,
            self.stats.time_in_seconds,
            // self.stats.typing
            "You're pretty bad"
        );

        let text_paragraph = Paragraph::new(text)
            .centered()
            .block(Block::default().padding(Padding::new(16, 16, 16, 16)))
            .style(Style::new().add_modifier(Modifier::BOLD))
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(text_paragraph, self.layout.top);
        self.render_footer(frame);
    }

    // Lorem ipsum
    // ^
    // Lorem ipsum

    fn calculate_result(&mut self) -> () {
        let mut typed_iter = self.typed_text.chars();
        let mut typing_stats = TypingStats::default();
        let mut current_word_correct_typed_count = 0;
        let mut word_has_error = false;

        for expected in self.text.chars() {
            if let Some(actual) = typed_iter.next() {
                if expected == actual {
                    typing_stats.correct += 1;
                    if expected.is_whitespace() {
                        typing_stats.typed += 1;
                        if !word_has_error {
                            typing_stats.typed += current_word_correct_typed_count;
                        }
                        current_word_correct_typed_count = 0;
                        word_has_error = false;
                    } else {
                        if !word_has_error {
                            current_word_correct_typed_count += 1;
                        }
                    }
                } else {
                    if expected.is_whitespace() {
                        typing_stats.extra += 1;
                        word_has_error = false;
                    } else {
                        typing_stats.misstyped += 1;
                        word_has_error = true;
                    }
                    current_word_correct_typed_count = 0;
                }
            }
        }
        typing_stats.typed += current_word_correct_typed_count;

        let incorrect = self.stats.typing.typed_raw - typing_stats.correct;
        let mut accuracy =
            (1 as f32 - (incorrect as f32 / typing_stats.correct as f32)) * 100 as f32;

        if accuracy < 0.0 {
            accuracy = 0.0;
        }

        let mut time_in_seconds = self
            .stats
            .finished
            .unwrap()
            .duration_since(self.stats.started.unwrap())
            .as_secs();

        if time_in_seconds == 0 {
            time_in_seconds = 1;
        }

        let wpm = (typing_stats.typed / 5) as f32 * (60 / time_in_seconds) as f32;

        typing_stats.incorrect = incorrect;
        typing_stats.typed_raw = self.stats.typing.typed_raw;
        self.stats.typing = typing_stats;
        self.stats.accuracy = accuracy;
        self.stats.wpm = wpm;
        self.stats.time_in_seconds = time_in_seconds;
    }

    fn handle_events(&mut self) -> Result<EventResult> {
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

        let event_result = self.handle_key_event(key_event);

        match event_result.action {
            Action::Quit => self.should_quit = true,
            Action::Insert => {
                if let (false, Some(t)) = (self.show_result, event_result.token) {
                    self.typed_text.push(t.into());
                    self.stats.typing.typed_raw += 1;
                }
            }
            Action::Restart => self.reset(),
            Action::None => (),
            Action::Delete => {
                let new_len = self
                    .typed_text
                    .char_indices()
                    .nth_back(0)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                self.typed_text.truncate(new_len);
            }
        }

        if self.show_result {
            return Ok(event_result);
        };

        let typed_len = self.typed_text.len();

        if typed_len == 0 {
            return Ok(event_result);
        }

        if typed_len >= self.text.len() {
            let expected_char = self.text.chars().nth(typed_len - 1);

            if expected_char.is_none() {
                panic!("Expected char is None. {typed_len} {}", self.text.len());
            }

            self.show_result = true;
            self.stats.finished = Some(Instant::now());
            return Ok(EventResult {
                action: Action::None,
                token: None,
            });
        } else {
            if self.stats.started.is_none() {
                self.stats.started = Some(Instant::now());
            }
            let expected_char = self.text.chars().nth(typed_len - 1);

            if expected_char.is_none() {
                panic!("Expected char is None. {typed_len} {}", self.text.len());
            }
        };

        Ok(event_result)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> EventResult {
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
            KeyCode::Char('r') => {
                if key_event.modifiers.contains(KeyModifiers::ALT) {
                    event_result.action = Action::Restart;
                } else {
                    event_result.action = Action::Insert;
                    event_result.token = Some(Token::Char('r'));
                }
            }
            KeyCode::Enter => {
                event_result.token = Some(Token::NewLine);
            }
            KeyCode::Backspace => {
                if self.typed_text.len() > 0 {
                    event_result.action = Action::Delete
                }
            }
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
}

fn main() -> Result<()> {
    let mut terminal = setup_terminal()?;
    let app = App::new(&terminal);
    app.unwrap().run(&mut terminal)?;
    teardown_terminal()?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn teardown_terminal() -> Result<()> {
    let mut stdout = io::stdout();
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
