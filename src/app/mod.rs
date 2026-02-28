use crate::{
    PARAGRAPH,
    input::{Action, EventResult},
};
use std::time::Instant;

#[derive(Default, Copy, Clone)]
pub struct Stats {
    pub correct: u16,
    pub incorrect: u16,
    pub typed: u16,
    pub misstyped: u16,
    pub extra: u16,
    pub accuracy: f32,
    pub wpm: f32,
    pub time_in_seconds: u64,
}

#[derive(Default, Debug)]
pub struct TypingSession {
    pub typed_text: String,
    pub started: Option<Instant>,
    pub finished: Option<Instant>,
    typed_raw: u16,
}

pub struct App {
    pub should_quit: bool,
    pub stats: Option<Stats>,
    pub current_session: TypingSession,
}

impl App {
    pub fn new() -> App {
        App {
            should_quit: false,
            stats: None,
            current_session: TypingSession::default(),
        }
    }

    pub fn reset(&mut self) {
        self.current_session.typed_text.clear();
        self.should_quit = false;
        self.stats = None;
    }

    pub fn results(&self) -> &Option<Stats> {
        &self.stats
    }

    pub fn update_results(&mut self) {
        if self.stats.is_some() {
            return;
        }

        self.stats = self.calculate_result();
    }

    fn calculate_result(&self) -> Option<Stats> {
        let mut typed_iter = self.current_session.typed_text.chars();
        let mut stats = Stats::default();
        let mut current_word_correct_typed_count = 0;
        let mut word_has_error = false;

        for expected in PARAGRAPH.chars() {
            if let Some(actual) = typed_iter.next() {
                if expected == actual {
                    stats.correct += 1;
                    if expected.is_whitespace() {
                        stats.typed += 1;
                        if !word_has_error {
                            stats.typed += current_word_correct_typed_count;
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
                        stats.extra += 1;
                        word_has_error = false;
                    } else {
                        stats.misstyped += 1;
                        word_has_error = true;
                    }
                    current_word_correct_typed_count = 0;
                }
            }
        }
        stats.typed += current_word_correct_typed_count;

        let incorrect = self.current_session.typed_raw - stats.correct;
        let mut accuracy = (1 as f32 - (incorrect as f32 / stats.correct as f32)) * 100 as f32;

        if accuracy < 0.0 {
            accuracy = 0.0;
        }

        let mut time_in_seconds = self
            .current_session
            .finished
            .unwrap()
            .duration_since(self.current_session.started.unwrap())
            .as_secs();

        if time_in_seconds == 0 {
            time_in_seconds = 1;
        }

        let wpm = (stats.typed / 5) as f32 * (60 / time_in_seconds) as f32;

        stats.incorrect = incorrect;
        stats.wpm = wpm;
        stats.accuracy = accuracy;
        stats.time_in_seconds = time_in_seconds;

        Some(stats)
    }

    pub fn update(&mut self, event_result: &EventResult) -> () {
        match event_result.action {
            Action::Quit => self.should_quit = true,
            Action::Insert => {
                if let (None, Some(t)) = (self.stats, event_result.token) {
                    self.current_session.typed_text.push(t.into());
                    self.current_session.typed_raw += 1;
                }
            }
            Action::Restart => self.reset(),
            Action::None => (),
            Action::Delete => {
                let new_len = self
                    .current_session
                    .typed_text
                    .char_indices()
                    .nth_back(0)
                    .map(|(i, _)| i)
                    .unwrap_or(0);

                self.current_session.typed_text.truncate(new_len);
            }
        }

        if self.stats.is_some() {
            return;
        };

        let typed_len = self.current_session.typed_text.len();

        if typed_len == 0 {
            return;
        }

        if typed_len == PARAGRAPH.len() {
            self.current_session.finished = Some(Instant::now());
            self.update_results();
        }

        if self.current_session.started.is_none() {
            self.current_session.started = Some(Instant::now());
        }
    }
}
