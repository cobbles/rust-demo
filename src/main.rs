extern crate rand;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::{thread_rng, Rng};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Gauge},
    Frame, Terminal,
};

struct App {
    node1: u16,
    node2: u16,
    node3: u16,
    node4: u16,
}

impl App {
    fn new() -> App {
        App {
            node1: 0,
            node2: 0,
            node3: 0,
            node4: 0,
        }
    }

    fn on_tick(&mut self) {
        self.node1 = thread_rng().gen_range(0..100);
        self.node2 = thread_rng().gen_range(0..100);
        self.node3 = thread_rng().gen_range(0..100);
        self.node4 = thread_rng().gen_range(0..100);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(2000);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
                if modifiers == KeyModifiers::CONTROL && KeyCode::Char('c') == code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
                Constraint::Percentage(50),
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
