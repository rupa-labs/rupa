use std::time::Duration;

/// Configuration for property transitions.
pub struct Transition {
    pub duration: Duration,
    pub delay: Duration,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            delay: Duration::ZERO,
        }
    }
}
