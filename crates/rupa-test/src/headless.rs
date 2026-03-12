use rupa_signals::Signal;
use rupa_context::{Registry, with_registry};
use std::sync::Arc;

/// A headless environment for testing reactive logic and components.
pub struct Tester {
    pub registry: Arc<Registry>,
}

impl Tester {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Registry::new()),
        }
    }

    /// Runs a closure within the test context.
    pub fn run<F, R>(&self, f: F) -> R
    where F: FnOnce() -> R {
        with_registry(self.registry.clone(), f)
    }

    /// Verifies that a signal changes as expected.
    pub fn assert_signal<T, F>(&self, signal: Signal<T>, action: F, expected: T)
    where 
        T: PartialEq + std::fmt::Debug + Clone + Send + Sync + 'static,
        F: FnOnce(&Signal<T>)
    {
        action(&signal);
        assert_eq!(signal.get(), expected);
    }
}
