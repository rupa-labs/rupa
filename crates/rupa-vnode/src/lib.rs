pub mod style;

pub use style::*;
pub use style::motion_mod as motion;
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
    
    #[serde(skip)]
    pub handlers: EventHandlers,
    
    pub motion: Option<motion_mod::Motion>,
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
            handlers: EventHandlers::default(),
            motion: None,
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

    pub fn with_style(mut self, style: Style) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.style = style;
        }
        self
    }

    pub fn with_handler(mut self, handler: impl Fn(UIEvent) + Send + Sync + 'static) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.handlers.on_click = Some(std::sync::Arc::new(handler));
        }
        self
    }

    pub fn with_motion(mut self, motion: motion_mod::Motion) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.motion = Some(motion);
        }
        self
    }

    pub fn with_child(mut self, child: VNode) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.children.push(child);
        }
        self
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.attributes.insert(key, value);
        }
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.key = Some(key.into());
        }
        self
    }
}
