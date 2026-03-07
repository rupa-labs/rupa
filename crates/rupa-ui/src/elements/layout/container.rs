use rupa_core::{Style, generate_id, Theme, Vec2};
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- CHILDREN HELPER ---
pub struct Children<'a> { pub items: Vec<Box<dyn Component + 'a>> }
impl<'a> Children<'a> {
    pub fn new() -> Self { Self { items: vec![] } }
    pub fn add(&mut self, child: Box<dyn Component + 'a>) { self.items.push(child); }
    pub fn get_all(&self) -> Vec<&dyn Component> { self.items.iter().map(|c| c.as_ref()).collect() }
    pub fn layout_all(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer) -> Vec<NodeId> {
        self.items.iter().map(|child| child.layout(taffy, measurer, None)).collect()
    }
    pub fn paint_all(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, _parent: NodeId, is_group_hovered: bool, global_pos: Vec2, _z_index: u32) {
        for child in &self.items {
            let node = child.get_node().unwrap().raw();
            let layout = taffy.layout(node).unwrap();
            let child_pos = global_pos + Vec2::new(layout.location.x, layout.location.y);
            child.paint(renderer, taffy, node, is_group_hovered, child_pos);
        }
    }
}

// --- CONTAINER ---
pub struct ContainerLogic<'a> { pub children: Children<'a> }
pub struct ContainerView { pub core: ViewCore }
pub struct Container<'a> { pub id: String, pub logic: ContainerLogic<'a>, pub view: ContainerView }
impl<'a> Container<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: ContainerLogic { children: Children::new() }, view: ContainerView { core: ViewCore::new(style) } }
    }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.logic.children.add(child); self.view.core.mark_dirty(); self }
}
impl<'a> Stylable for Container<'a> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for Container<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        let child_nodes = self.logic.children.layout_all(taffy, measurer);
        taffy.set_children(node, &child_nodes).unwrap();
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let style_ref = self.view.core.style.read().unwrap();
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style_ref.is_group, global_pos, 0);
    }
}
