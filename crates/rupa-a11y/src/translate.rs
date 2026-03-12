use rupa_vnode::VNode;
use crate::node::{Node, Role};

/// Translates a Rupa VNode into an accessibility Node.
pub fn translate_vnode(node: &VNode) -> Option<Node> {
    match node {
        VNode::Element(el) => {
            let role = match el.tag.as_str() {
                "button" => Role::Button,
                "h1" | "h2" | "h3" => Role::Heading,
                "input" => Role::TextInput,
                "img" => Role::Image,
                "a" => Role::Link,
                _ => Role::Label,
            };

            let mut a11y = Node::new(role);
            
            // Extract label from attributes or style if possible
            if let Some(label) = el.attributes.map.get("aria-label") {
                a11y = a11y.with_label(label);
            }

            Some(a11y)
        }
        VNode::Text(t) => {
            Some(Node::new(Role::Label).with_label(t))
        }
        _ => None,
    }
}
