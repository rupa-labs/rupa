//! # Rupa Error 🚨
//!
//! System-wide error models and diagnostic reporting for the Rupa Framework.

use thiserror::Error;
use std::sync::Arc;

/// The primary error type for the Rupa Framework.
///
/// Encapsulates failures from layout, rendering, reactive systems, and platform-specific drivers.
#[derive(Error, Debug, Clone, PartialEq)]
pub enum Error {
    /// Failure within the Taffy layout engine.
    #[error("Layout engine failure: {0}")]
    Layout(String),

    /// Failure within the WGPU renderer or shader pipeline.
    #[error("GPU/Renderer failure: {0}")]
    Renderer(String),

    /// Failure within the OS-specific shell (Winit, Android NDK, etc).
    #[error("OS Platform failure: {0}")]
    Platform(String),

    /// Requested feature is not supported on the current target.
    #[error("Unsupported feature: {0}")]
    Unsupported(String),

    /// A specific component failed during initialization or rendering.
    #[error("Component '{id}' failed: {message}")]
    Component { id: String, message: String },

    /// Asset or resource loading failed.
    #[error("Resource loading failed: {0}")]
    Resource(String),

    /// The reactive system reached an inconsistent or circular state.
    #[error("Reactive system out of sync: {0}")]
    Reactivity(String),

    /// An unhandled panic was caught and recovered.
    #[error("Panic caught in {location}: {message}")]
    Panic { location: String, message: String },

    /// An application-specific or custom extension error.
    #[error("Custom error: {0}")]
    Custom(String),
}

pub type Result<T> = std::result::Result<T, Error>;

/// Global diagnostic hub for reporting and handling framework errors.
pub struct DiagnosticCenter {
    /// Thread-safe error handler callback.
    pub handler: Arc<dyn Fn(Error) + Send + Sync>,
}

impl DiagnosticCenter {
    /// Creates a new DiagnosticCenter with a default logger-based handler.
    pub fn new() -> Self {
        Self {
            handler: Arc::new(|err| {
                log::error!("[RUPA DIAGNOSTIC] {}", err);
            }),
        }
    }

    /// Reports an error to the registered handler.
    pub fn report(&self, error: Error) {
        (self.handler)(error);
    }
}

impl Default for DiagnosticCenter {
    fn default() -> Self {
        Self::new()
    }
}
