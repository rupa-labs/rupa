use rupa_vnode::VNode;

/// Utilities for verifying VNode tree structures.
pub struct Snapshot;

impl Snapshot {
    /// Asserts that a VNode matches a specific tag and child count.
    pub fn assert_element(node: &VNode, tag: &str, children_count: usize) {
        if let VNode::Element(el) = node {
            assert_eq!(el.tag, tag, "Tag mismatch");
            assert_eq!(el.children.len(), children_count, "Children count mismatch");
        } else {
            panic!("Expected VNode::Element, found {:?}", node);
        }
    }

    /// Asserts that a VNode is a text node with specific content.
    pub fn assert_text(node: &VNode, content: &str) {
        if let VNode::Text(text) = node {
            assert_eq!(text, content, "Text content mismatch");
        } else {
            panic!("Expected VNode::Text, found {:?}", node);
        }
    }
}
