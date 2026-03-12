//! # Rupa UI 🎨
//!
//! High-level semantic UI system for the Rupa Framework. This crate provides 
//! the **Composites** for platform-agnostic components, styling modifiers, 
//! and layout primitives.

pub mod elements;
pub mod primitives;
pub mod style;
pub mod body;
pub mod prelude;

pub use body::Body;
pub use elements::Children;
use rupa_vnode::VNode;

/// A mock helper for UI testing in headless environments.
pub struct MockUI;

impl MockUI {
    /// Helper to render a component into a VNode for structural assertion.
    pub fn render_to_node(node: VNode) -> VNode {
        node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rupa_vnode::VNode;

    #[test]
    fn test_mock_ui_render() {
        let node = VNode::element("div");
        let rendered = MockUI::render_to_node(node.clone());
        assert_eq!(rendered, node);
    }
}
