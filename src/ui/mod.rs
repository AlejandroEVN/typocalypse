use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

use crate::{HELP, PARAGRAPH, app::App};

struct AppLayout {
    top: Rect,
    bottom: Rect,
}

pub struct UI {
    layout: AppLayout,
}

impl UI {
    pub fn new(width: u16, height: u16) -> UI {
        let area = Rect::new(0, 0, width, height);
        let (areas, _) = Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)])
            .split_with_spacers(area);
        let layout = AppLayout {
            top: areas[0],
            bottom: areas[1],
        };

        UI { layout }
    }

    fn render_footer(&self, frame: &mut Frame, app: &App) {
        let mut footer_content = format!("{} - Current: {}", HELP, 0);

        if let Some(stats) = app.stats {
            footer_content = format!("{} - Total: {} secs", HELP, stats.time_in_seconds);
        } else if let Some(started) = app.current_session.started {
            footer_content = format!("{} - Current: {} secs", HELP, started.elapsed().as_secs());
        }

        let footer = Paragraph::new(footer_content);

        frame.render_widget(footer, self.layout.bottom);
    }

    fn render_result(&self, frame: &mut Frame, app: &App) {
        let results_option = app.results();

        let text = match results_option {
            Some(results) => {
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
            }
            None => "Error loading your results".to_string(),
        };

        let text_paragraph = Paragraph::new(text)
            .centered()
            .block(Block::default().padding(Padding::new(16, 16, 16, 16)))
            .style(Style::new().add_modifier(Modifier::BOLD))
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(text_paragraph, self.layout.top);
        self.render_footer(frame, app);
    }

    pub fn update(&self, frame: &mut Frame, app: &mut App) {
        if app.stats.is_some() {
            self.render_result(frame, app);
            return;
        }

        let mut spans = Vec::<Span>::new();
        let mut typed_iter = app.current_session.typed_text.chars();
        let mut lines = Vec::<Line>::new();

        for expected in PARAGRAPH.chars() {
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
        self.render_footer(frame, app);
    }
}
