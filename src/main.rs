use std::{
    io{self, Stdout},
    thread,
    time::{Duration, Instant},
    sync::mpsc,
};
use crossterm::{
    event::{self, EnableMouseCapture, DisableMouseCapture, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::*,
    layout::{Layout, Constraint, Direction},
    style::{Color, Style},
    Terminal,
}

fn main() -> Result<(), io::Error> {
    // Set up terminal
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // User event handler
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Render loop
    loop {
        // TODO: get sysinfo

        // TODO: draw terminal
        todo!();

        // TODO: handle user input
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
