use rupa_styling::{Style, Attributes};
use serde::{Serialize, Deserialize};

/// The Universal UI Node of the Rupa Framework.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VNode {
    /// A structural element (e.g., div, button).
    Element(VElement),
    /// Textual content.
    Text(String),
    /// A placeholder for a component.
    Component(VComponent),
    /// A collection of nodes without a wrapper element.
    Fragment(Vec<VNode>),
    /// An empty node for conditional rendering.
    Empty,
}

/// A virtual representation of a platform-agnostic UI element.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VElement {
    pub tag: String,
    pub style: Style,
    pub attributes: Attributes,
    pub children: Vec<VNode>,
    pub key: Option<String>,
}

/// Metadata for a lazily-resolved component in the VNode tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VComponent {
    pub name: String,
    pub props: serde_json::Value,
}

impl VNode {
    pub fn element(tag: impl Into<String>) -> Self {
        VNode::Element(VElement {
            tag: tag.into(),
            style: Style::default(),
            attributes: Attributes::default(),
            children: Vec::new(),
            key: None,
        })
    }

    pub fn text(content: impl Into<String>) -> Self {
        VNode::Text(content.into())
    }

    pub fn fragment(children: Vec<VNode>) -> Self {
        VNode::Fragment(children)
    }
}
