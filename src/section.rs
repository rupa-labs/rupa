use crate::utils::{Style, StyleModifier, generate_id, Theme};
use crate::Component;
use crate::container::Children;
use crate::renderer::Renderer;
use taffy::prelude::*;

/// Represents a distinct semantic part of the interface.
pub struct Section {
    pub id: String,
    pub name: String,
    pub style: Style,
    pub children: Children,
}

impl Section {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        
        Self {
            id: generate_id(),
            name: name.into(),
            style,
            children: Children::new(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }

    pub fn children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children.append(children);
        self
    }
}

impl Component for Section {
    fn id(&self) -> &str { &self.id }

    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(self.style.to_taffy(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        self.children.layout_all(taffy, node);
        node
    }

    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let layout = taffy.layout(node).unwrap();
        let style = if is_group_hovered && self.style.group_hover.is_some() { self.style.group_hover.as_ref().unwrap() } else { &self.style };
        if let Some(color) = style.background.color.clone() {
            renderer.draw_rect(layout.location.x, layout.location.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw);
        }
        self.children.paint_all(renderer, taffy, node, is_group_hovered || self.style.is_group, render_pass);
    }

    fn on_click(&self) {}
    fn on_scroll(&self, _delta: f32) {}
    fn on_drag(&self, _delta: crate::utils::Vec2) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_section_creation() {
        let section = Section::new("Sidebar").id("side-1");
        assert_eq!(section.name, "Sidebar");
        assert_eq!(section.id, "side-1");
    }
}
