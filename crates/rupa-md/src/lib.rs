//! # Rupa Markdown (MD) 📖
//!
//! Content-driven engine for the Rupa Framework. This crate provides 
//! the **Composites** for transforming Markdown/MDX into universal `VNode` trees.

use pulldown_cmark::{Parser, Event, Tag, Options, HeadingLevel};
pub use rupa_vnode::{VNode, VElement, Attributes, Style};
use std::collections::HashMap;

/// Configuration for the Markdown Engine.
/// 
/// Enables fine-grained control over how semantic tags are mapped to 
/// specific visual styles and attributes.
#[derive(Clone, Default)]
pub struct Config {
    /// Custom styles for specific HTML-like tags (e.g., "h1", "p", "a").
    pub tag_styles: HashMap<String, Style>,
    /// Global style applied to the root fragment container.
    pub root_style: Style,
}

/// The central Markdown Orchestrator.
pub struct Engine;

impl Engine {
    /// Parses a markdown string into a platform-agnostic `VNode` tree.
    ///
    /// # Examples
    ///
    /// ```
    /// use rupa_md::Engine;
    /// let vnode = Engine::parse("# Hello Rupa", None);
    /// ```
    pub fn parse(content: &str, config: Option<&Config>) -> VNode {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);

        let parser = Parser::new_ext(content, options);
        let mut root_children = Vec::new();
        let mut stack: Vec<VElement> = Vec::new();

        for event in parser {
            match event {
                Event::Start(tag) => {
                    let mut attributes = Attributes::default();
                    let vtag = match tag {
                        Tag::Heading { level, .. } => {
                            let l = match level {
                                HeadingLevel::H1 => 1,
                                HeadingLevel::H2 => 2,
                                HeadingLevel::H3 => 3,
                                HeadingLevel::H4 => 4,
                                HeadingLevel::H5 => 5,
                                HeadingLevel::H6 => 6,
                            };
                            format!("h{}", l)
                        },
                        Tag::Paragraph => "p".to_string(),
                        Tag::List(None) => "ul".to_string(),
                        Tag::List(Some(_)) => "ol".to_string(),
                        Tag::Item => "li".to_string(),
                        Tag::Emphasis => "em".to_string(),
                        Tag::Strong => "strong".to_string(),
                        Tag::Link { dest_url, .. } => {
                            attributes.insert("href", dest_url.to_string());
                            "a".to_string()
                        },
                        Tag::Image { dest_url, .. } => {
                            attributes.insert("src", dest_url.to_string());
                            "img".to_string()
                        },
                        Tag::CodeBlock(_) => "pre".to_string(),
                        Tag::Table(_) => "table".to_string(),
                        Tag::TableHead => "thead".to_string(),
                        Tag::TableRow => "tr".to_string(),
                        Tag::TableCell => "td".to_string(),
                        _ => "div".to_string(),
                    };

                    let style = config
                        .and_then(|c| c.tag_styles.get(&vtag))
                        .cloned()
                        .unwrap_or_default();

                    stack.push(VElement {
                        tag: vtag,
                        style,
                        attributes,
                        handlers: Default::default(),
                        motion: None,
                        children: Vec::new(),
                        key: None,
                    });
                }
                Event::End(_) => {
                    if let Some(finished_el) = stack.pop() {
                        let node = VNode::Element(finished_el);
                        if let Some(parent) = stack.last_mut() {
                            parent.children.push(node);
                        } else {
                            root_children.push(node);
                        }
                    }
                }
                Event::Text(text) => {
                    let node = VNode::text(text.to_string());
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        root_children.push(node);
                    }
                }
                Event::Code(code) => {
                    let node = VNode::Element(VElement {
                        tag: "code".to_string(),
                        style: Style::default(),
                        attributes: Attributes::default(),
                        handlers: Default::default(),
                        motion: None,
                        children: vec![VNode::text(code.to_string())],
                        key: None,
                    });
                    if let Some(parent) = stack.last_mut() {
                        parent.children.push(node);
                    } else {
                        root_children.push(node);
                    }
                }
                _ => {}
            }
        }

        VNode::Fragment(root_children)
    }
}

/// A mock helper for testing Markdown rendering in headless environments.
pub struct MockContent;

impl MockContent {
    /// Generates a simple VNode fragment for testing structural expectations.
    pub fn simple_h1(text: &str) -> VNode {
        VNode::fragment(vec![
            VNode::element("h1").with_child(VNode::text(text))
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_parsing() {
        let node = Engine::parse("# Rupa", None);
        if let VNode::Fragment(children) = node {
            assert_eq!(children.len(), 1);
            if let VNode::Element(ref el) = children[0] {
                assert_eq!(el.tag, "h1");
            }
        } else {
            panic!("Should be a fragment");
        }
    }
}
