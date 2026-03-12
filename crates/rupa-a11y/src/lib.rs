//! # Rupa Accessibility (a11y) ♿
//!
//! Semantic Accessibility and Screen Reader integration for the Rupa Framework. 
//! Defines the foundational traits (Ports) for assistive technology communication.

pub mod bridge;
pub mod node;
pub mod translate;

pub use bridge::{Bridge, Port};
pub use node::{Node, Role};

use rupa_signals::Signal;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// The central manager for application accessibility state.
///
/// Responsible for maintaining the global accessibility tree and 
/// dispatching updates to platform-specific bridges (Screen Readers).
pub struct Manager {
    /// Reactive map of all accessible nodes in the tree.
    pub nodes: Signal<HashMap<String, Node>>,
    /// Optional platform-specific bridge (e.g., AccessKit, Web A11y).
    pub bridge: Option<Port>,
}

impl Manager {
    /// Creates a new, empty accessibility manager.
    pub fn new() -> Self {
        Self {
            nodes: Signal::new(HashMap::new()),
            bridge: None,
        }
    }

    /// Fluent API: Attaches a platform-specific accessibility bridge.
    pub fn with_bridge(mut self, bridge: Port) -> Self {
        self.bridge = Some(bridge);
        self
    }

    /// Updates or inserts an accessible node into the tree.
    pub fn update_node(&self, id: impl Into<String>, node: Node) {
        self.nodes.update(|map| {
            map.insert(id.into(), node);
        });
    }
}

/// A mock implementation of the Accessibility Bridge for TDD and headless testing.
pub struct MockBridge {
    pub last_announcement: Arc<RwLock<Option<String>>>,
}

impl Bridge for MockBridge {
    fn announce(&self, message: &str) {
        *self.last_announcement.write().unwrap() = Some(message.to_string());
    }

    fn update_tree(&self, _nodes: &HashMap<String, Node>) {
        // Mock tree update
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_accessibility_flow() {
        let bridge = Arc::new(MockBridge {
            last_announcement: Arc::new(RwLock::new(None)),
        });
        
        bridge.announce("Hello Artisan");
        
        let announcement = bridge.last_announcement.read().unwrap();
        assert_eq!(announcement.as_ref().unwrap(), "Hello Artisan");
    }
}
