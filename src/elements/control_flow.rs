use crate::utils::{Signal, generate_id};
use crate::Component;
use crate::renderer::Renderer;
use taffy::prelude::*;
use std::fmt::Debug;

pub struct Show { pub id: String, pub when: Signal<bool>, pub child: Box<dyn Component> }
impl Show { pub fn new(when: Signal<bool>, child: Box<dyn Component>) -> Self { Self { id: generate_id(), when, child } } }
impl Component for Show {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        if self.when.get() { self.child.layout(taffy, parent) }
        else { let node = taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::None, ..Default::default() }).unwrap(); if let Some(p) = parent { taffy.add_child(p, node).unwrap(); } node }
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) { if self.when.get() { self.child.paint(renderer, taffy, node, is_group_hovered, render_pass); } }
    fn on_click(&self) { if self.when.get() { self.child.on_click(); } }
    fn on_scroll(&self, d: f32) { if self.when.get() { self.child.on_scroll(d); } }
    fn on_drag(&self, d: crate::utils::Vec2) { if self.when.get() { self.child.on_drag(d); } }
}

pub struct ForEach<T: Clone + Debug + 'static> { pub id: String, pub items: Signal<Vec<T>>, pub render_item: Box<dyn Fn(&T) -> Box<dyn Component>> }
impl<T: Clone + Debug + 'static> ForEach<T> { pub fn new(items: Signal<Vec<T>>, render_item: impl Fn(&T) -> Box<dyn Component> + 'static) -> Self { Self { id: generate_id(), items, render_item: Box::new(render_item) } } }
impl<T: Clone + Debug + 'static> Component for ForEach<T> {
    fn id(&self) -> &str { &self.id }
    fn layout(&self, taffy: &mut TaffyTree<()>, parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_with_children(taffy::style::Style::default(), &[]).unwrap();
        if let Some(p) = parent { taffy.add_child(p, node).unwrap(); }
        for item in self.items.get().iter() { (self.render_item)(item).layout(taffy, Some(node)); }
        node
    }
    fn paint(&self, renderer: &mut Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, render_pass: &mut wgpu::RenderPass<'_>) {
        let children_nodes = taffy.children(node).unwrap();
        for (i, item) in self.items.get().iter().enumerate() { if let Some(child_node) = children_nodes.get(i) { (self.render_item)(item).paint(renderer, taffy, *child_node, is_group_hovered, render_pass); } }
    }
    fn on_click(&self) {}
    fn on_scroll(&self, _: f32) {}
    fn on_drag(&self, _: crate::utils::Vec2) {}
}
