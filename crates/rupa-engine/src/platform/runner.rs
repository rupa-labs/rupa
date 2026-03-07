use rupa_core::error::Error;
use crate::platform::app::AppMetadata;

#[derive(Debug)]
pub enum PlatformEvent {
    RequestRedraw,
}

/// The common interface for all platform execution shells.
pub trait PlatformRunner {
    /// Synchronizes application metadata with the host environment.
    fn sync_metadata(&self, metadata: &AppMetadata) -> Result<(), Error>;

    /// Starts the application's main loop.
    fn run(self) -> Result<(), Error>;
}
