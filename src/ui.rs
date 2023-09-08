use tui::{
    backend::Backend,
    layout::Alignment,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{self, Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Padding, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::App;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui-org/ratatui/tree/master/examples

    // Create a vertical layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Percentage(80),
            ]
            .as_ref(),
        )
        .split(frame.size());

    // Render the block
    render_scramble(app, frame, main_chunks[0]);
    render_basic_stats(app, frame, main_chunks[1]);
    render_stats_timer(app, frame, main_chunks[2]);
    if app.show_help {
        let area = centered_rect(40, 40, frame.size());

        frame.render_widget(Clear, area);
        frame.render_widget(help_msg(), area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn help_msg() -> Paragraph<'static> {
    let popup_block = Block::default()
        .title(Span::styled(
            "Help",
            Style::default().fg(Color::LightYellow),
        ))
        .borders(Borders::ALL);

    let text = vec![
        text::Line::from(vec![
            Span::styled("Space: ", Style::default().fg(Color::LightYellow)),
            Span::from("Start/Stop timer"),
        ]),
        text::Line::from(vec![
            Span::styled("q, Ctrl-C, Esc: ", Style::default().fg(Color::LightYellow)),
            Span::from("Quit"),
        ]),
        text::Line::from(vec![
            Span::styled("h: ", Style::default().fg(Color::LightYellow)),
            Span::from("Toggle help"),
        ]),
        text::Line::from(vec![
            Span::styled("s: ", Style::default().fg(Color::LightYellow)),
            Span::from("New scramble"),
        ]),
        text::Line::from(vec![
            Span::styled("d: ", Style::default().fg(Color::LightYellow)),
            Span::from("Delete last time"),
        ]),
        text::Line::from(vec![
            Span::styled("r: ", Style::default().fg(Color::LightYellow)),
            Span::from("Reset"),
        ]),
        text::Line::from(vec![
            Span::styled("l: ", Style::default().fg(Color::LightYellow)),
            Span::from("Toggle last scramble"),
        ]),
        text::Line::from(vec![
            Span::styled("f: ", Style::default().fg(Color::LightYellow)),
            Span::from("Add/remove DNF penalty to last time"),
        ]),
        text::Line::from(vec![
            Span::styled("2: ", Style::default().fg(Color::LightYellow)),
            Span::from("Add/remove +2 penalty to last time"),
        ]),
    ];
    return Paragraph::new(text)
        .block(popup_block)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);
}

// Render scramble
fn render_scramble<B: Backend>(app: &mut App, frame: &mut Frame<B>, area: Rect) {
    let scramble_block = Block::default()
        .title(Span::styled(
            "Scramble",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let scramble_p = Paragraph::new(app.scramble.to_string())
        .block(scramble_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(scramble_p, area);
}

// Render current/best stats
fn render_basic_stats<B: Backend>(app: &mut App, frame: &mut Frame<B>, area: Rect) {
    let stats_block = Block::default()
        .title(Span::styled(
            "Stats",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let stats = match app.show_last_scramble {
        true => {
            vec![text::Line::from(vec![
                Span::styled("Last scramble: ", Style::default().fg(Color::LightYellow)),
                Span::from(app.last_scramble()),
            ])]
        }

        false => {
            vec![text::Line::from(vec![
                Span::styled("Current: ", Style::default().fg(Color::LightYellow)),
                Span::from("Single: "),
                Span::from(app.times.currents()[0].to_string()),
                Span::from(" | ao5: "),
                Span::from(app.times.currents()[1].to_string()),
                Span::from(" | ao12: "),
                Span::from(app.times.currents()[2].to_string()),
                Span::from("    "),
                Span::styled("Best: ", Style::default().fg(Color::LightYellow)),
                Span::from("Single: "),
                Span::from(app.times.bests()[0].to_string()),
                Span::from(" | ao5: "),
                Span::from(app.times.bests()[1].to_string()),
                Span::from(" | ao12: "),
                Span::from(app.times.bests()[2].to_string()),
            ])]
        }
    };

    let stats_p = Paragraph::new(stats)
        .block(stats_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(stats_p, area);
}

// Render stats/timer
fn render_stats_timer<B: Backend>(app: &mut App, frame: &mut Frame<B>, area: Rect) {
    let stats_timer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(area);

    render_stats_table(app, frame, stats_timer_layout[0]);
    render_timer(app, frame, stats_timer_layout[1]);
}

// Render stats table
fn render_stats_table<B: Backend>(app: &mut App, frame: &mut Frame<B>, area: Rect) {
    let stats_table_block = Block::default()
        .title(Span::styled(
            "Stats",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .padding(Padding::new(1, 1, 1, 1));

    let rows = app.times.times["times"]
        .as_array()
        .unwrap()
        .iter()
        .rev()
        .map(|t| {
            Row::new(vec![
                Line::from(format!("{:.3}", t["time"].as_f64().unwrap_or(0.0) as f64))
                    .alignment(Alignment::Center),
                Line::from(
                    app.times
                        .display_time(t["ao5"].as_f64().unwrap_or(0.0) as f64),
                )
                .alignment(Alignment::Center),
                Line::from(
                    app.times
                        .display_time(t["ao12"].as_f64().unwrap_or(0.0) as f64),
                )
                .alignment(Alignment::Center),
            ])
            .bottom_margin(1)
            .style(
                Style::default().fg(match t["penalty"].as_str().unwrap_or("NA") {
                    "DNF" => Color::Red,
                    "+2" => Color::Yellow,
                    _ => Color::White,
                }),
            )
        });

    let stats_table = Table::new(rows)
        .style(Style::default().fg(Color::White))
        .header(
            Row::new(vec!["Single", "ao5", "ao12"])
                .style(Style::default().fg(Color::LightYellow))
                .bottom_margin(1),
        )
        .block(stats_table_block)
        .column_spacing(2)
        .highlight_style(
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">>")
        .widths(&[
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ]);

    frame.render_widget(stats_table, area);
}

// Render timer
fn render_timer<B: Backend>(app: &mut App, frame: &mut Frame<B>, area: Rect) {
    let timer_block = Block::default()
        .title(Span::styled(
            "Timer",
            Style::default()
                .fg(Color::LightYellow)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Left)
        .padding(Padding::new(1, 1, 5, 1))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    let timer_p = Paragraph::new(app.time_string())
        .block(timer_block)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(app.time_color)
                .add_modifier(Modifier::BOLD),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(timer_p, area);
}
