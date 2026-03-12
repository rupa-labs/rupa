//! # Rupa Test 🧪
//!
//! Testing utilities and headless infrastructure for the Rupa Framework. 
//! Provides the foundational Atoms for TDD, snapshots, and interaction simulation.

pub mod headless;
pub mod snapshot;
pub mod interaction;

pub use headless::Tester;
pub use snapshot::Snapshot;
use std::sync::{Arc, RwLock};

/// A helper to quickly setup a fresh, headless test environment.
pub fn setup() -> Tester {
    Tester::new()
}

/// A mock implementation of UI interactions for TDD and headless testing.
pub struct MockInteraction {
    pub events_fired: Arc<RwLock<usize>>,
}

impl MockInteraction {
    /// Creates a new, empty mock interaction tracker.
    pub fn new() -> Self {
        Self {
            events_fired: Arc::new(RwLock::new(0)),
        }
    }

    /// Simulates a UI event firing.
    pub fn fire_event(&self) {
        let mut count = self.events_fired.write().unwrap();
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_interaction_flow() {
        let interaction = MockInteraction::new();
        interaction.fire_event();
        
        let count = interaction.events_fired.read().unwrap();
        assert_eq!(*count, 1);
    }
}
