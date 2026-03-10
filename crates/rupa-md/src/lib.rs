use pulldown_cmark::{Parser, Event, Tag, Options, HeadingLevel};
use rupa_vnode::{VNode, VElement, Attributes, Style};

pub struct MarkdownEngine;

impl MarkdownEngine {
    /// Parses a markdown string into a VNode tree.
    pub fn parse(content: &str) -> VNode {
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
                        Tag::Link { .. } => "a".to_string(),
                        Tag::Image { .. } => "img".to_string(),
                        Tag::CodeBlock(_) => "pre".to_string(),
                        Tag::Table(_) => "table".to_string(),
                        Tag::TableHead => "thead".to_string(),
                        Tag::TableRow => "tr".to_string(),
                        Tag::TableCell => "td".to_string(),
                        _ => "div".to_string(),
                    };

                    stack.push(VElement {
                        tag: vtag,
                        style: Style::default(),
                        attributes: Attributes::default(),
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
