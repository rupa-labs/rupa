use rupa_core::component::Component;
use rupa_core::renderer::{Renderer, TextMeasurer};
use rupa_core::Vec2;
use taffy::prelude::*;

pub struct Children<'a> {
    pub items: Vec<Box<dyn Component + 'a>>,
}

impl<'a> Children<'a> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn push(&mut self, child: Box<dyn Component + 'a>) {
        self.items.push(child);
    }

    pub fn add(&mut self, child: Box<dyn Component + 'a>) {
        self.items.push(child);
    }

    pub fn as_refs(&self) -> Vec<&dyn Component> {
        self.items.iter().map(|c| c.as_ref()).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Component + 'a>> {
        self.items.iter()
    }

    pub fn layout_all(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer) -> Vec<NodeId> {
        self.items.iter().map(|c| c.layout(taffy, measurer, None)).collect()
    }

    pub fn paint_all(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, _parent_node: NodeId, is_group_hovered: bool, global_pos: Vec2, _z_index: i32) {
        for child in self.items.iter() {
            if let Some(node) = child.get_node() {
                child.paint(renderer, taffy, node.raw(), is_group_hovered, global_pos);
            }
        }
    }
}
