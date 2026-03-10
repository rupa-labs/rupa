use rupa_core::Component;
use rupa_vnode::VNode;

pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Renders a Rupa component into an HTML string.
    pub fn render_to_string(root: &dyn Component) -> String {
        let vnode = root.render();
        Self::render_vnode(&vnode)
    }

    /// Recursively converts a VNode tree into an HTML string.
    pub fn render_vnode(node: &VNode) -> String {
        match node {
            VNode::Element(el) => {
                let mut html = format!("<{}", el.tag);
                
                // Attributes
                for (key, value) in &el.attributes.map {
                    html.push_str(&format!(" {}=\"{}\"", key, value));
                }

                // Inline Styles (Basic)
                if !el.style.is_default() {
                    html.push_str(" style=\"");
                    // Minimal style serialization for SSR
                    if let Some(ref bg) = el.style.background.color {
                        html.push_str(&format!("background-color: {};", bg.to_hex()));
                    }
                    html.push_str("\"");
                }

                html.push('>');

                // Children
                for child in &el.children {
                    html.push_str(&Self::render_vnode(child));
                }

                html.push_str(&format!("</{}>", el.tag));
                html
            }
            VNode::Text(text) => text.clone(),
            VNode::Fragment(children) => {
                children.iter().map(|c| Self::render_vnode(c)).collect::<Vec<_>>().join("")
            }
            VNode::Empty => "".to_string(),
            VNode::Component(comp) => {
                // In a real SSR, we might need to resolve the component or render its placeholder
                format!("<!-- Component: {} -->", comp.name)
            }
        }
    }
}

pub mod axum_handler {
    // Axum integration for serving SSR requests
}
