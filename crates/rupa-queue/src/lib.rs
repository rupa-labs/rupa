//! # Rupa Queue ⛓️
//!
//! Background task orchestration and systemic coordination for the Rupa Framework. 
//! Provides the foundational traits (Ports) for asynchronous task management.

pub mod task;
pub mod queue;

use std::sync::{Arc, RwLock};
pub use task::{Task, Status, Handle};
pub use queue::Queue;

/// The primary Port for background task execution.
///
/// This trait must be implemented by concrete Showroom Adapters (e.g., Tokio, WASM Workers).
pub type Port = Arc<dyn Task>;

/// A mock implementation of the Queue for TDD and headless testing.
pub struct MockQueue {
    pub tasks_executed: Arc<RwLock<usize>>,
}

impl MockQueue {
    /// Creates a new, empty mock queue.
    pub fn new() -> Self {
        Self {
            tasks_executed: Arc::new(RwLock::new(0)),
        }
    }

    /// Increments the executed task count.
    pub fn increment_tasks(&self) {
        let mut count = self.tasks_executed.write().unwrap();
        *count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_queue_basic() {
        let queue = MockQueue::new();
        queue.increment_tasks();
        
        let count = queue.tasks_executed.read().unwrap();
        assert_eq!(*count, 1);
    }
}
