use crossterm::{
    event::{self, Event},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, // Trait for the `execute` method on stdout below
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    terminal::Terminal,
};
use std::{
    io::{self, stdout, Error},
    time::Duration,
};

pub fn init() -> Result<Terminal<impl Backend>, Error> {
    let terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    Ok(terminal)
}

pub fn restore() -> Result<(), Error> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

pub fn next_event(timeout: Duration) -> Result<Option<Event>, Error> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    let event = event::read()?;
    Ok(Some(event))
}
