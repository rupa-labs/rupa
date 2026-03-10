use rupa_signals::Signal;

/// High-level state management for UI inputs.
pub struct Form<T> {
    pub data: Signal<T>,
    pub is_dirty: Signal<bool>,
    pub is_valid: Signal<bool>,
}

impl<T: Clone + Send + Sync + 'static> Form<T> {
    pub fn new(initial: T) -> Self {
        Self {
            data: Signal::new(initial),
            is_dirty: Signal::new(false),
            is_valid: Signal::new(true),
        }
    }
}
