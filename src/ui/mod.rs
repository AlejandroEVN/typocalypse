use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Padding, Paragraph, Wrap},
};

use crate::app::{App, Menu, Totals};

pub struct UI {}

impl UI {
    pub fn new() -> Self {
        UI {}
    }

    fn render_footer(&self, frame: &mut Frame, app: &App) {
        let (key_binding, text) = match app.selected_menu {
            Menu::Home => ("[⌥s] ", "Stats   "),
            Menu::Stats => ("[esc] ", "Home   "),
        };

        let footer = Line::from(vec![
            Span::styled("[⌥q] ", Style::default().fg(Color::Red)),
            Span::styled(
                "Quit   ",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::DarkGray),
            ),
            Span::styled("[⌥r] ", Style::default().fg(Color::Yellow)),
            Span::styled(
                "Restart   ",
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::DarkGray),
            ),
            Span::styled(key_binding, Style::default().fg(Color::Green)),
            Span::styled(
                text,
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::DarkGray),
            ),
        ]);

        let footer = Paragraph::new(footer)
            .block(Block::new().dark_gray())
            .centered();

        let (_, bottom) = Self::layout(frame.area());
        frame.render_widget(footer, bottom);
    }

    fn render_historic(&self, frame: &mut Frame, app: &mut App) {
        let avg_stats = app.get_historic_stats();

        self.render_stats(frame, &avg_stats, "--- Avg results ---", true);

        self.render_footer(frame, app);
    }

    fn render_result(&self, frame: &mut Frame, app: &App) {
        self.render_stats(frame, &app.results(), "--- Last results ---", false);

        self.render_footer(frame, &app);
    }

    fn render_stats(&self, frame: &mut Frame, totals: &Totals, title: &str, show_count: bool) {
        let left_padding = 15;
        let mut labels = vec![
            (
                "WPM",
                format!(
                    "{:>left_padding$}",
                    format!("{}", totals.avg_stats.wpm.round())
                ),
                Color::Yellow,
            ),
            (
                "ACC",
                format!(
                    "{:>left_padding$}",
                    format!("{:.2}%", totals.avg_stats.accuracy)
                ),
                Color::Green,
            ),
            (
                "TIME",
                format!(
                    "{:>left_padding$}",
                    format!("{}s", totals.avg_stats.time_in_seconds)
                ),
                Color::Cyan,
            ),
        ];

        if show_count {
            labels.push((
                "TESTS",
                format!("{:>left_padding$}", format!("{}", totals.count)),
                Color::Magenta,
            ));
        }

        let stat_lines: Vec<Line> = labels
            .iter()
            .map(|(label, value, color)| {
                Line::from(vec![
                    Span::styled(format!("{:<6}", label), Style::new().fg(*color)),
                    Span::styled(
                        value.clone(),
                        Style::new().fg(*color).add_modifier(Modifier::BOLD),
                    ),
                ])
            })
            .collect();

        let mut lines = vec![
            Line::from(title).centered().style(
                Style::new()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::DarkGray),
            ),
            Line::from(""),
        ];

        lines.extend(stat_lines);
        lines.extend(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    format!("{} typed", totals.avg_stats.typed),
                    Style::new().fg(Color::Green),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("{} mistyped", totals.avg_stats.misstyped),
                    Style::new().fg(Color::Red),
                ),
                Span::raw("  "),
                Span::styled(
                    format!("{} extra", totals.avg_stats.extra),
                    Style::new()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::DarkGray),
                ),
            ]),
        ]);

        let text_paragraph = Paragraph::new(lines)
            .centered()
            .block(Block::default().padding(Padding::new(16, 16, 16, 16)))
            .wrap(ratatui::widgets::Wrap { trim: true });

        let (top, _) = Self::layout(frame.area());
        frame.render_widget(text_paragraph, top);
    }

    pub fn update(&self, frame: &mut Frame, app: &mut App, text: &str) {
        match app.selected_menu {
            Menu::Home => {
                if app.stats.is_some() {
                    self.render_result(frame, app);
                    return;
                }
            }
            Menu::Stats => {
                self.render_historic(frame, app);
                return;
            }
        }

        let mut spans = Vec::<Span>::new();
        let mut typed_iter = app.current_session.typed_text.chars();
        let mut lines = Vec::<Line>::new();

        for expected in text.chars() {
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
        self.render_footer(frame, &app);
    }

    fn layout(area: Rect) -> (Rect, Rect) {
        let areas =
            Layout::vertical([Constraint::Percentage(95), Constraint::Percentage(5)]).split(area);

        (areas[0], areas[1])
    }
}
