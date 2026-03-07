use rupa_core::{Style, generate_id, Theme, Vec2};
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- SHOW ---
pub struct Show<'a> { pub id: String, pub logic: ShowLogic<'a>, pub view: ShowView }
pub struct ShowLogic<'a> { pub when: bool, pub child: Box<dyn Component + 'a> }
pub struct ShowView { pub core: ViewCore }
impl<'a> Show<'a> {
    pub fn new(when: bool, child: Box<dyn Component + 'a>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: ShowLogic { when, child }, view: ShowView { core: ViewCore::new(style) } }
    }
}
impl<'a> Stylable for Show<'a> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for Show<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { if self.logic.when { vec![self.logic.child.as_ref()] } else { vec![] } }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        if self.logic.when { self.logic.child.layout(taffy, measurer, Some(node)); }
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        if self.logic.when { self.logic.child.paint(renderer, taffy, node, is_group_hovered, global_pos); }
    }
}

// --- FOREACH ---
pub struct ForEach<'a, T> { pub id: String, pub logic: ForEachLogic<'a, T>, pub view: ForEachView }
pub struct ForEachLogic<'a, T> { pub items: Vec<T>, pub builder: Box<dyn Fn(&T) -> Box<dyn Component + 'a> + Send + Sync> }
pub struct ForEachView { pub core: ViewCore }
impl<'a, T> ForEach<'a, T> {
    pub fn new(items: Vec<T>, builder: impl Fn(&T) -> Box<dyn Component + 'a> + Send + Sync + 'static) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: ForEachLogic { items, builder: Box::new(builder) }, view: ForEachView { core: ViewCore::new(style) } }
    }
}
impl<'a, T: Send + Sync> Stylable for ForEach<'a, T> { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl<'a, T: Send + Sync> Component for ForEach<'a, T> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] } // ForEach is a special component, needs better handling
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
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}
