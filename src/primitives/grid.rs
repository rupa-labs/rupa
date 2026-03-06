use crate::support::{Style, generate_id, Vec2, Display};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::dispatcher::UIEvent;
use crate::scene::SceneNode;
use crate::elements::layout::container::Children;
use taffy::prelude::*;
use std::cell::{RefMut};

// --- LOGIC ---
pub struct GridLogic<'a> {
    pub children: Children<'a>,
}

// --- VIEW ---
pub struct GridView {
    pub core: ViewCore,
}

impl GridView {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = Display::Grid;
        Self { core: ViewCore::new(style) }
    }
}

// --- COMPONENT ---
pub struct Grid<'a> {
    pub id: String,
    pub logic: GridLogic<'a>,
    pub view: GridView,
}

impl<'a> Grid<'a> {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            logic: GridLogic { children: Children::new() },
            view: GridView::new(),
        }
    }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.add(c); self.view.core.mark_dirty(); self }
}

impl<'a> Stylable for Grid<'a> {
    fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() }
}

impl<'a> Component for Grid<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { 
                taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); 
            }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };

        if let Some(p) = parent {
            let current_children = taffy.children(p).unwrap_or_default();
            if !current_children.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }

        self.logic.children.layout_all(taffy, measurer, node);
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.borrow();
        if let Some(color) = style_ref.background.color.clone() {
            renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw);
        }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }

    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}
