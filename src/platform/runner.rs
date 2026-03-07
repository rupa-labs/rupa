use crate::support::error::RupauiError;

#[derive(Debug)]
pub enum PlatformEvent {
    RequestRedraw,
}

/// The common interface for all platform execution shells.
pub trait PlatformRunner {
    /// Starts the application's main loop.
    fn run(self) -> Result<(), RupauiError>;
}
