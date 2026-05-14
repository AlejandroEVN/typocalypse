use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

use crate::app::App;

const HELP: &str = "Press something";

pub struct UI;

impl UI {
    pub fn new() -> Self {
        UI
    }

    fn render_footer(&self, frame: &mut Frame, app: &App) {
        let mut footer_content = format!("{} - Current: {}", HELP, 0);

        if let Some(stats) = app.stats {
            footer_content = format!("{} - Total: {} secs", HELP, stats.time_in_seconds);
        } else if let Some(started) = app.current_session.started {
            footer_content = format!("{} - Current: {} secs", HELP, started.elapsed().as_secs());
        }

        let footer = Paragraph::new(footer_content);

        let (_, bottom) = Self::layout(frame.area());
        frame.render_widget(footer, bottom);
    }

    fn render_result(&self, frame: &mut Frame, app: &App) {
        let text = if let Some(results) = app.results() {
            format!(
                "WPM: {} | {}/{}/{} | Accuracy: {} | Seconds: {} \n {:?}",
                results.wpm,
                results.typed,
                results.misstyped,
                results.extra,
                results.accuracy,
                results.time_in_seconds,
                "Placeholder"
            )
        } else {
            "Error loading your results".to_string()
        };

        let text_paragraph = Paragraph::new(text)
            .centered()
            .block(Block::default().padding(Padding::new(16, 16, 16, 16)))
            .style(Style::new().add_modifier(Modifier::BOLD))
            .wrap(ratatui::widgets::Wrap { trim: true });

        let (top, _) = Self::layout(frame.area());
        frame.render_widget(text_paragraph, top);
        self.render_footer(frame, app);
    }

    pub fn update(&self, frame: &mut Frame, app: &mut App, text: &str) {
        if app.stats.is_some() {
            self.render_result(frame, app);
            return;
        }

        let mut spans = Vec::<Span>::new();
        let mut typed_iter = app.current_session.typed_text.chars();
        let mut lines = Vec::<Line>::new();

        for expected in text.chars() {
            match expected {
                _ => {
                    let (span_text, style): (String, Style) = match typed_iter.next() {
                        Some(actual) => {
                            if expected == actual {
                                if actual == '\n' {
                                    ("\\n".to_string(), Style::default().fg(Color::Green))
                                } else {
                                    (expected.to_string(), Style::default().fg(Color::Green))
                                }
                            } else {
                                if expected == ' ' {
                                    (
                                        ' '.to_string(),
                                        Style::default().bg(Color::Red).fg(Color::White),
                                    )
                                } else if expected == '\n' {
                                    (
                                        "\\n".to_string(),
                                        Style::default().bg(Color::Red).fg(Color::White),
                                    )
                                } else {
                                    (expected.to_string(), Style::default().fg(Color::Red))
                                }
                            }
                        }
                        None => (expected.to_string(), Style::default()),
                    };

                    let span = Span::styled(span_text, style);

                    if expected == '\n' {
                        spans.push(span);
                        lines.push(Line::from(spans));
                        spans = Vec::new();
                    } else {
                        spans.push(span);
                    }
                }
            }
        }

        if !spans.is_empty() {
            lines.push(Line::from(spans));
        }

        let text = Text::from(lines);
        let text_paragraph = Paragraph::new(text)
            .block(Block::default().padding(Padding::new(16, 16, 16, 16)))
            .style(Style::new().add_modifier(Modifier::BOLD))
            .wrap(Wrap { trim: true });

        let (top, _) = Self::layout(frame.area());
        frame.render_widget(text_paragraph, top);
        self.render_footer(frame, app);
    }

    fn layout(area: Rect) -> (Rect, Rect) {
        let areas =
            Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)]).split(area);

        (areas[0], areas[1])
    }
}
