use rupa_core::VNode;
use taffy::prelude::*;
use unicode_width::UnicodeWidthStr;

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn build_taffy_from_vnode(node: &VNode, taffy: &mut TaffyTree<()>) -> NodeId {
        match node {
            VNode::Element(el) => {
                let mut style = el.style.to_taffy();
                if style.size.width == Dimension::Auto { style.size.width = Dimension::Percent(1.0); }
                if el.tag == "input" {
                    style.size.height = Dimension::Length(1.0);
                    style.padding = taffy::prelude::Rect {
                        left: length(1.0), right: length(1.0), top: length(0.0), bottom: length(0.0)
                    };
                }
                
                let children: Vec<NodeId> = el.children.iter()
                    .map(|c| Self::build_taffy_from_vnode(c, taffy))
                    .collect();
                
                taffy.new_with_children(style, &children).unwrap()
            }
            VNode::Text(text) => {
                let mut style = taffy::prelude::Style::default();
                style.size.width = Dimension::Length(text.width() as f32);
                style.size.height = Dimension::Length(1.0);
                taffy.new_leaf(style).unwrap()
            }
            VNode::Fragment(children) => {
                let mut style = taffy::prelude::Style::default();
                style.display = Display::Flex;
                let children: Vec<NodeId> = children.iter()
                    .map(|c| Self::build_taffy_from_vnode(c, taffy))
                    .collect();
                taffy.new_with_children(style, &children).unwrap()
            }
            _ => taffy.new_leaf(taffy::prelude::Style::default()).unwrap(),
        }
    }
}
