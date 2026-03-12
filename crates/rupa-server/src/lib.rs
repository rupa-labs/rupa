//! # Rupa Server 🌍
//!
//! The Backend Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) to render Rupa applications 
//! into static HTML for Server-Side Rendering (SSR) and Static Site Generation (SSG).

use rupa_core::Component;
use rupa_vnode::VNode;

/// A high-performance HTML renderer for generating static markup from the VNode tree.
pub struct HtmlRenderer;

impl HtmlRenderer {
    /// Renders a Rupa component into an HTML string.
    ///
    /// Useful for generating the initial page load on the server.
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

                // Inline Styles (Basic translation for critical CSS)
                if !el.style.is_default() {
                    html.push_str(" style=\"");
                    // Note: A real implementation would serialize the entire Style struct into CSS properties
                    if el.style.background.color.is_some() {
                        html.push_str("background-color: var(--rupa-bg);");
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

/// Web server integrations.
pub mod axum_handler {
    // Axum integration for serving SSR requests
}
