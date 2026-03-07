use crate::support::{Style, generate_id, Theme, Vec2};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- BRAND ---

pub struct BrandLogic { pub name: String }
pub struct BrandView { pub core: ViewCore }
pub struct Brand { pub id: String, pub logic: BrandLogic, pub view: BrandView }

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: BrandLogic { name: name.into() }, view: BrandView { core: ViewCore::new(style) } }
    }
}

impl Stylable for Brand { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }

impl Component for Brand {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}
