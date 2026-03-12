//! # Rupa VNode 🌳
//!
//! The Universal UI Contract of the Rupa Framework. 
//! Defines the platform-agnostic representation of UI elements, text, and components.

pub mod style;

pub use style::*;
pub use style::motion_mod as motion;
use serde::{Serialize, Deserialize};

/// The Universal UI Node of the Rupa Framework.
///
/// `VNode` is a serializable representation of a UI tree. It can represent 
/// physical elements, raw text, fragments, or component placeholders.
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
///
/// Elements contain styling, attributes, event handlers, and children.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VElement {
    /// The semantic tag of the element (e.g., "div", "stack", "button").
    pub tag: String,
    /// The visual style of the element.
    pub style: Style,
    /// Platform-specific attributes.
    pub attributes: Attributes,
    
    /// Interactive event handlers. Skip serialization as closures cannot be serialized.
    #[serde(skip)]
    pub handlers: EventHandlers,
    
    /// Optional motion/animation rules.
    pub motion: Option<motion_mod::Motion>,
    /// Nested child nodes.
    pub children: Vec<VNode>,
    /// Unique key for efficient reconciliation.
    pub key: Option<String>,
}

/// Metadata for a lazily-resolved component in the VNode tree.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VComponent {
    /// The registered name of the component.
    pub name: String,
    /// JSON-serialized props for the component.
    pub props: serde_json::Value,
}

impl VNode {
    /// Creates a new Element node.
    ///
    /// # Examples
    ///
    /// ```
    /// use rupa_vnode::VNode;
    /// let node = VNode::element("div");
    /// ```
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

    /// Creates a new Text node.
    pub fn text(content: impl Into<String>) -> Self {
        VNode::Text(content.into())
    }

    /// Creates a new Fragment node.
    pub fn fragment(children: Vec<VNode>) -> Self {
        VNode::Fragment(children)
    }

    /// Fluent API: Assigns a style to the element.
    pub fn with_style(mut self, style: Style) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.style = style;
        }
        self
    }

    /// Fluent API: Assigns a click handler to the element.
    pub fn with_handler(mut self, handler: impl Fn(UIEvent) + Send + Sync + 'static) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.handlers.on_click = Some(std::sync::Arc::new(handler));
        }
        self
    }

    /// Fluent API: Assigns motion rules to the element.
    pub fn with_motion(mut self, motion: motion_mod::Motion) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.motion = Some(motion);
        }
        self
    }

    /// Fluent API: Adds a child node to the element.
    pub fn with_child(mut self, child: VNode) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.children.push(child);
        }
        self
    }

    /// Fluent API: Adds an attribute to the element.
    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.attributes.insert(key, value);
        }
        self
    }

    /// Fluent API: Assigns a reconciliation key to the element.
    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        if let VNode::Element(ref mut el) = self {
            el.key = Some(key.into());
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vnode_builder_pattern() {
        let node = VNode::element("div")
            .with_key("main")
            .with_attr("id", "app")
            .with_child(VNode::text("Hello Rupa"));

        if let VNode::Element(el) = node {
            assert_eq!(el.tag, "div");
            assert_eq!(el.key, Some("main".to_string()));
            assert_eq!(el.attributes.get("id"), Some(&"app".to_string()));
            assert_eq!(el.children.len(), 1);
            assert_eq!(el.children[0], VNode::text("Hello Rupa"));
        } else {
            panic!("Node should be an element");
        }
    }

    #[test]
    fn test_fragment_creation() {
        let frag = VNode::fragment(vec![
            VNode::text("A"),
            VNode::text("B"),
        ]);
        
        if let VNode::Fragment(children) = frag {
            assert_eq!(children.len(), 2);
        } else {
            panic!("Node should be a fragment");
        }
    }
}
