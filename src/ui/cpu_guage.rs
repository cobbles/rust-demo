use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge}, Frame, backend::Backend
};

use crate::types;

pub fn render_guages<B: Backend>(f: &mut Frame<B>, app: &types::App, area: Rect) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(0)
    .constraints(
        [
            Constraint::Max(2),
            Constraint::Max(2),
            Constraint::Max(2)
        ]
        .as_ref(),
    )
    .split(area);

    let mut count = 0;
    for node in &app.nodes {
        let gauge = cpu_guage(
        node.name.to_string(),
        node.cpu_percentage
        );
        f.render_widget(gauge, chunks[count]);
        count += 1;
    }
}

pub fn cpu_guage(title: String, percent: u16) -> Gauge<'static> {
    return Gauge::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::LEFT | Borders::RIGHT),
        )
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(percent);
}
