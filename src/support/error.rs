use thiserror::Error;
use std::sync::Arc;

#[derive(Error, Debug, Clone)]
pub enum RupauiError {
    #[error("Layout engine failure: {0}")]
    Layout(String),

    #[error("GPU/Renderer failure: {0}")]
    Renderer(String),

    #[error("OS Platform failure: {0}")]
    Platform(String),

    #[error("Component '{id}' failed: {message}")]
    Component { id: String, message: String },

    #[error("Resource loading failed: {0}")]
    Resource(String),

    #[error("Reactive system out of sync: {0}")]
    Reactivity(String),

    #[error("Panic caught in {location}: {message}")]
    Panic { location: String, message: String },
}

pub type Result<T> = std::result::Result<T, RupauiError>;

/// Global error subscriber for the framework.
pub struct DiagnosticCenter {
    pub handler: Arc<dyn Fn(RupauiError) + Send + Sync>,
}

impl DiagnosticCenter {
    pub fn report(&self, error: RupauiError) {
        (self.handler)(error);
    }
}
