use rupa_signals::Signal;

/// The central orchestrator for application navigation.
pub struct Router {
    pub current_path: Signal<String>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            current_path: Signal::new("/".to_string()),
        }
    }
}
