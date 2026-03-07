use std::io::{stdout, Stdout};
use std::time::Duration;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, size},
};

/// A wrapper for Terminal operations using Crossterm.
/// Isolates native terminal calls for better SOC.
pub struct TerminalInterface {
    pub stdout: Stdout,
}

impl TerminalInterface {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn setup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
        Ok(())
    }

    pub fn restore(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(self.stdout, event::DisableMouseCapture, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        size().unwrap_or((80, 24))
    }

    pub fn poll_event(&self, timeout: Duration) -> Result<Option<Event>, Box<dyn std::error::Error>> {
        if event::poll(timeout)? {
            return Ok(Some(event::read()?));
        }
        Ok(None)
    }
}
