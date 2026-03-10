use rupa_vnode::{VNode, Style, Attributes};
use serde::{Serialize, Deserialize};

/// An atomic instruction for the renderer to update the UI state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Patch {
    /// Create a new element and append it to a parent.
    Create {
        parent_id: Option<String>,
        node: VNode,
        index: usize,
    },
    /// Update attributes or styles of an existing node.
    Update {
        id: String,
        changes: Vec<UpdateType>,
    },
    /// Move an existing node to a new index.
    Move {
        id: String,
        from_index: usize,
        to_index: usize,
    },
    /// Remove a node from the tree.
    Delete {
        id: String,
    },
    /// Replace an entire sub-tree.
    Replace {
        id: String,
        new_node: VNode,
    }
}

/// Specific types of updates that can be applied to a node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UpdateType {
    /// Change the style of the node.
    Style(Style),
    /// Add or update an attribute.
    Attribute(String, String),
    /// Remove an attribute.
    RemoveAttribute(String),
    /// Change the text content (for Text nodes).
    Text(String),
}
