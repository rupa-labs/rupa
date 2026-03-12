use crate::node::Node;
use std::collections::HashMap;
use std::sync::Arc;

/// The primary Port for connecting the Rupa UI tree to OS Accessibility APIs.
/// Adapters (Tier 3) will implement this (e.g. using AccessKit).
pub trait Bridge: Send + Sync {
    /// Updates the OS with the current accessibility tree state.
    fn update_tree(&self, nodes: HashMap<String, Node>);
    
    /// Notifies the OS that a specific node has been focused.
    fn focus_node(&self, id: &str);

    /// Commands the OS Screen Reader to announce a message.
    fn announce(&self, message: &str);
}

/// A handle to the active accessibility bridge.
pub type Port = Arc<dyn Bridge>;
