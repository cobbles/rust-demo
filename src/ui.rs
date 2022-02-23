use tui::{
    backend::{Backend},
    layout::{Constraint, Direction, Layout},
    Frame,
};

use crate::types;

use self::cpu_guage::render_guages;

mod cpu_guage;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &types::App) {
    let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .margin(0)
    .constraints(
        [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]
        .as_ref(),
    )
    .split(f.size());
    render_guages(f, app, chunks[0]);
    render_guages(f, app, chunks[1]);
}
