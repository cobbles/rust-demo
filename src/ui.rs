use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Frame,
};

use crate::types;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &types::App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("node-10-10-0-1")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(app.node1);
    f.render_widget(gauge, chunks[0]);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("node-10-10-0-2")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(app.node2);
    f.render_widget(gauge, chunks[1]);

    let gauge = Gauge::default()
        .block(
            Block::default()
                .title("node-10-10-0-3")
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(app.node3);
    f.render_widget(gauge, chunks[2]);
}
