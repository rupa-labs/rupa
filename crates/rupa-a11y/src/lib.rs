pub mod bridge;
pub mod node;
pub mod translate;

pub use bridge::{Bridge, Port};
pub use node::{Node, Role};

use rupa_signals::Signal;
use std::collections::HashMap;

/// The central manager for application accessibility state.
pub struct Manager {
    pub nodes: Signal<HashMap<String, Node>>,
    pub bridge: Option<Port>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            nodes: Signal::new(HashMap::new()),
            bridge: None,
        }
    }

    pub fn with_bridge(mut self, bridge: Port) -> Self {
        self.bridge = Some(bridge);
        self
    }

    pub fn update_node(&self, id: impl Into<String>, node: Node) {
        self.nodes.update(|map: &mut HashMap<String, Node>| {
            map.insert(id.into(), node);
        });
    }
}
