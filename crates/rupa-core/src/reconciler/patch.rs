use rupa_vnode::{VNode, Style};
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
    /// Change the complete style of the node.
    Style(Style),
    /// Change a specific sub-style of the node (e.g., Layout, Spacing).
    StylePart(StylePart),
    /// Add or update an attribute.
    Attribute(String, String),
    /// Remove an attribute.
    RemoveAttribute(String),
    /// Change the text content (for Text nodes).
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StylePart {
    Layout(rupa_vnode::style::layout::Layout),
    Flex(rupa_vnode::style::flex::Flex),
    Grid(rupa_vnode::style::grid::Grid),
    Sizing(rupa_vnode::style::sizing::Sizing),
    Spacing(rupa_vnode::style::spacing::Spacing),
    Padding(rupa_vnode::style::spacing::Spacing),
    Margin(rupa_vnode::style::spacing::Spacing),
    Background(rupa_vnode::style::background::Background),
    Border(rupa_vnode::style::border::Border),
    Rounding(rupa_vnode::style::border::Rounding),
    Outline(rupa_vnode::style::border::Outline),
    Typography(rupa_vnode::style::typography::TypographyStyle),
    Interactivity(rupa_vnode::style::interactivity::Interactivity),
    Shadow(Option<rupa_vnode::style::effects::Shadow>),
    Filter(Option<rupa_vnode::style::filters::Filter>),
    Motion(Option<rupa_vnode::style::motion::Motion>),
}

/// The result of a reconciliation process.
pub type PatchSet = Vec<Patch>;
