use crate::{
    db::DB,
    input::{Action, EventResult},
};
use std::time::Instant;

#[derive(Default, Copy, Clone, Debug)]
pub struct Stats {
    pub correct: u16,
    pub incorrect: u16,
    pub typed: u16,
    pub misstyped: u16,
    pub extra: u16,
    pub accuracy: f32,
    pub wpm: f32,
    pub time_in_seconds: i64,
}

#[derive(Default, Clone, Debug)]
pub struct Totals {
    pub avg_stats: Stats,
    pub count: u16,
    pub all_results: Option<Vec<Stats>>,
}

#[derive(Default, Debug)]
pub struct TypingSession {
    pub typed_text: String,
    pub started: Option<Instant>,
    pub finished: Option<Instant>,
    typed_raw: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Menu {
    Home,
    Stats,
}

pub struct App<'a> {
    text: &'a str,
    db: DB,
    pub should_quit: bool,
    pub stats: Option<Stats>,
    pub current_session: TypingSession,
    pub selected_menu: Menu,
    pub historic: Totals,
    calculate_historic: bool,
}

impl App<'_> {
    pub fn new(db: DB, text: &str) -> App<'_> {
        App {
            db,
            text,
            should_quit: false,
            stats: None,
            current_session: TypingSession::default(),
            selected_menu: Menu::Home,
            calculate_historic: true,
            historic: Totals::default(),
        }
    }

    pub fn reset(&mut self) {
        self.current_session = TypingSession::default();
        self.should_quit = false;
        self.stats = None;
    }

    pub fn results(&self) -> Totals {
        let stats = if let Some(current_stats) = self.stats {
            current_stats
        } else {
            eprintln!("Error loading your results");
            std::process::exit(1);
        };

        Totals {
            avg_stats: stats,
            count: 1,
            all_results: None,
        }
    }

    pub fn update_results(&mut self) {
        if self.stats.is_some() {
            return;
        }

        self.stats = self.calculate_result();
    }

    fn calculate_result(&mut self) -> Option<Stats> {
        let mut typed_iter = self.current_session.typed_text.chars();
        let mut stats = Stats::default();
        let mut current_word_correct_typed_count = 0;
        let mut word_has_error = false;

        for expected in self.text.chars() {
            if let Some(actual) = typed_iter.next() {
                if expected == actual {
                    stats.correct += 1;
                    if expected.is_whitespace() || expected == '\n' {
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
                    if expected.is_whitespace() || expected == '\n' {
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

        let wpm = (stats.typed as f32 / 5.0) * (60.0 / time_in_seconds as f32);

        stats.incorrect = incorrect;
        stats.wpm = wpm;
        stats.accuracy = accuracy;
        stats.time_in_seconds = time_in_seconds as i64;

        let _ = self.db.insert_results(stats);
        self.calculate_historic = true;

        Some(stats)
    }

    pub fn get_historic_stats(&mut self) -> &Totals {
        if !self.calculate_historic {
            return &self.historic;
        };

        let historic_results = self.db.get_results();
        let count = historic_results.len() as u16;

        if count == 0 {
            self.calculate_historic = false;
            return &self.historic;
        }

        let stats_sum = historic_results
            .iter()
            .fold(Stats::default(), |acc, e| Stats {
                correct: acc.correct + e.correct,
                incorrect: acc.incorrect + e.incorrect,
                typed: acc.typed + e.typed,
                misstyped: acc.misstyped + e.misstyped,
                extra: acc.extra + e.extra,
                accuracy: acc.accuracy + e.accuracy,
                wpm: acc.wpm + e.wpm,
                time_in_seconds: acc.time_in_seconds + e.time_in_seconds,
            });

        self.historic = Totals {
            avg_stats: Stats {
                correct: stats_sum.correct / count,
                incorrect: stats_sum.incorrect / count,
                typed: stats_sum.typed / count,
                misstyped: stats_sum.misstyped / count,
                extra: stats_sum.extra / count,
                accuracy: stats_sum.accuracy / count as f32,
                wpm: stats_sum.wpm / count as f32,
                time_in_seconds: stats_sum.time_in_seconds / count as i64,
            },
            all_results: Some(historic_results),
            count,
        };

        self.calculate_historic = false;
        &self.historic
    }

    fn handle_event_result(&mut self, event_result: &EventResult) {
        match event_result.action {
            Action::Quit => self.should_quit = true,
            Action::Insert => {
                if let (None, Some(t)) = (self.stats, event_result.token) {
                    self.current_session.typed_text.push(t.into());
                    self.current_session.typed_raw += 1;
                }
            }
            Action::SelectMenu => {
                if let Some(selected_menu) = &event_result.selected_menu {
                    self.selected_menu = selected_menu.clone();
                } else {
                    self.selected_menu = Menu::Home;
                }
            }
            Action::Restart => {
                if let Some(selected_menu) = &event_result.selected_menu {
                    self.selected_menu = selected_menu.clone();
                } else {
                    self.selected_menu = Menu::Home;
                };
                self.reset();
            }
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
    }

    pub fn update(&mut self, event_result: &EventResult) -> () {
        self.handle_event_result(event_result);

        if self.stats.is_some() {
            return;
        };

        let typed_len = self.current_session.typed_text.len();

        if typed_len == 0 {
            return;
        }

        if self.current_session.started.is_none() {
            self.current_session.started = Some(Instant::now());
        }

        if typed_len == self.text.len() {
            self.current_session.finished = Some(Instant::now());
            self.update_results();
        }
    }
}
