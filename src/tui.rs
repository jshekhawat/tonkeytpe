use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{
    app::{App, Screen, MENU_MODES},
    stats::Stats,
    theme::Theme,
    typing::GameMode,
};

pub fn render(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let bg = Style::default().bg(theme.bg).fg(theme.fg);
    frame.render_widget(Block::default().style(bg), frame.area());

    match app.screen {
        Screen::Menu => render_menu(frame, app),
        Screen::Typing => render_typing(frame, app),
        Screen::Results => render_results(frame, app),
    }
}

fn themed_block<'a>(title: &'a str, theme: &Theme) -> Block<'a> {
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.dim))
        .style(Style::default().bg(theme.bg).fg(theme.fg))
        .title(title)
}

fn render_menu(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    let h_center = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    let inner = h_center[1];
    let inner_v = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(14),
            Constraint::Min(0),
        ])
        .split(inner);

    let title_text = vec![
        Line::from(Span::styled(
            " ╔════════════════════════╗ ",
            Style::default().fg(theme.accent),
        )),
        Line::from(Span::styled(
            " ║     T O N K E Y T Y P E     ║ ",
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            " ╚════════════════════════╝ ",
            Style::default().fg(theme.accent),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "   a monkeytype-inspired typing trainer",
            Style::default().fg(theme.dim),
        )),
        Line::from(Span::styled(
            "   for the terminal",
            Style::default().fg(theme.dim),
        )),
    ];

    let title = Paragraph::new(Text::from(title_text)).alignment(Alignment::Center);
    frame.render_widget(title, inner_v[0]);

    let modes: Vec<ListItem> = MENU_MODES
        .iter()
        .enumerate()
        .flat_map(|(i, m)| {
            let mut items = vec![];
            if i == 4 {
                items.push(ListItem::new(Span::styled("", Style::default())));
            }
            let (prefix, style) = if i == app.menu_selection {
                ("▶", Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD))
            } else {
                (" ", Style::default().fg(theme.fg))
            };
            items.push(ListItem::new(Span::styled(
                format!("  {} {}", prefix, m.label),
                style,
            )));
            items
        })
        .collect();

    let list = List::new(modes);
    frame.render_widget(list, inner_v[1]);

    let help = Paragraph::new(format!(
        "enter: start  |  arrows/jk: navigate  |  tab: theme ({})  |  q/esc: quit",
        app.theme.name
    ))
        .style(Style::default().fg(theme.dim))
        .alignment(Alignment::Center);

    frame.render_widget(help, vertical[2]);
}

fn render_typing(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    let test = match app.typing_test.as_ref() {
        Some(t) => t,
        None => return,
    };

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    render_stats_bar(frame, vertical[0], test, theme);
    render_word_area(frame, vertical[1], test, theme);

    let status = if test.is_finished() {
        "Test complete! Enter for results."
    } else if !test.is_started() {
        "Start typing to begin..."
    } else {
        "esc: back to menu"
    };

    let help = Paragraph::new(status)
        .style(Style::default().fg(theme.dim))
        .alignment(Alignment::Center);
    frame.render_widget(help, vertical[2]);
}

fn render_stats_bar(
    frame: &mut Frame,
    area: Rect,
    test: &crate::typing::TypingTest,
    theme: &Theme,
) {
    let stats = Stats::calculate(test);

    let time_str = match test.mode {
        GameMode::Timed(_) => {
            let remaining = test.time_remaining();
            format!("{:.0}s", remaining)
        }
        GameMode::WordCount(_n) => {
            let (done, total) = test.word_progress();
            format!("{}/{}", done.min(total), total)
        }
    };

    let time_color = if test.time_remaining() < 5.0 && test.time_remaining() > 0.0 {
        theme.incorrect
    } else {
        theme.fg
    };

    let spans = vec![
        Span::styled(
            format!("{:.0}", stats.wpm),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" wpm  ", Style::default().fg(theme.dim)),
        Span::styled(
            format!("{:.0}%", stats.accuracy),
            Style::default()
                .fg(theme.correct)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" acc  ", Style::default().fg(theme.dim)),
        Span::styled(
            time_str,
            Style::default()
                .fg(time_color)
                .add_modifier(Modifier::BOLD),
        ),
    ];

    let p = Paragraph::new(Line::from(spans)).alignment(Alignment::Center);
    frame.render_widget(p, area);
}

fn render_word_area(
    frame: &mut Frame,
    area: Rect,
    test: &crate::typing::TypingTest,
    theme: &Theme,
) {
    let h_pad = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(8),
            Constraint::Percentage(84),
            Constraint::Percentage(8),
        ])
        .split(area);

    let inner = h_pad[1];
    let max_columns = inner.width as usize;
    let mut lines: Vec<Line> = Vec::new();
    let mut current_line: Vec<Span> = Vec::new();
    let mut line_width: usize = 0;

    let err_style = Style::default()
        .fg(theme.incorrect)
        .add_modifier(Modifier::BOLD);

    let cursor_style = Style::default()
        .fg(theme.bg)
        .bg(theme.cursor)
        .add_modifier(Modifier::BOLD);

    let correct_style = Style::default().fg(theme.correct);
    let default_style = Style::default().fg(theme.fg);

    for (i, &ch) in test.expected.iter().enumerate() {
        if line_width >= max_columns && ch == ' ' {
            lines.push(Line::from(std::mem::take(&mut current_line)));
            line_width = 0;
        }

        if i == test.correct_up_to {
            for err in &test.pending_errors {
                current_line.push(Span::styled(err.to_string(), err_style));
            }
            current_line.push(Span::styled(ch.to_string(), cursor_style));
        } else if i < test.correct_up_to {
            current_line.push(Span::styled(ch.to_string(), correct_style));
        } else {
            current_line.push(Span::styled(ch.to_string(), default_style));
        }

        line_width += 1;
    }

    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    frame.render_widget(Paragraph::new(lines), inner);
}

fn render_results(frame: &mut Frame, app: &App) {
    let theme = &app.theme;
    let area = frame.area();

    let stats = app.typing_test.as_ref().map(|t| Stats::calculate(t));

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(area);

    let content = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vertical[1]);

    if let Some(s) = stats {
        let stats_text = vec![
            Line::from(Span::styled(
                "RESULTS",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                format!("  wpm      {:.0}", s.wpm),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                format!("  accuracy {:.1}%", s.accuracy),
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!("  raw      {:.0}", s.raw_wpm),
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!("  time     {:.1}s", s.elapsed),
                Style::default().fg(theme.fg),
            )]),
            Line::from(vec![Span::styled(
                format!(
                    "  chars    {} correct / {} incorrect / {} total",
                    s.correct_chars, s.incorrect_chars, s.total_chars
                ),
                Style::default().fg(theme.fg),
            )]),
        ];

        let p = Paragraph::new(stats_text)
            .alignment(Alignment::Center)
            .block(themed_block("", theme));

        frame.render_widget(p, content[1]);
    }

    let help = Paragraph::new("enter: restart  |  esc/q: back to menu")
        .style(Style::default().fg(theme.dim))
        .alignment(Alignment::Center);

    frame.render_widget(help, vertical[2]);
}
