use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::support::vector::Vec2;
use crate::support::{Style, generate_id, StyleModifier, Accessibility, Attributes, Overflow, Signal, Theme};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::dispatcher::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Children<'a> { pub list: Vec<Box<dyn Component + 'a>> }
impl<'a> Children<'a> {
    pub fn new() -> Self { Self { list: Vec::new() } }
    pub fn add(&mut self, child: Box<dyn Component + 'a>) { self.list.push(child); }
    pub fn get_all(&self) -> Vec<&dyn Component> { self.list.iter().map(|c| c.as_ref()).collect() }
    pub fn layout_all(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: NodeId) -> Vec<NodeId> { 
        let child_nodes: Vec<NodeId> = self.list.iter().map(|child| child.layout(taffy, measurer, Some(parent))).collect();
        taffy.set_children(parent, &child_nodes).unwrap(); child_nodes
    }
    pub fn paint_all(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, parent_node: NodeId, is_group_hovered: bool, parent_global_pos: Vec2, start_index: usize) {
        if let Ok(children_nodes) = taffy.children(parent_node) {
            for (i, child) in self.list.iter().enumerate() {
                if let Some(node) = children_nodes.get(i + start_index) {
                    if let Ok(layout) = taffy.layout(*node) {
                        let global_pos = parent_global_pos + Vec2::new(layout.location.x, layout.location.y);
                        child.paint(renderer, taffy, *node, is_group_hovered, global_pos);
                    }
                }
            }
        }
    }
}

pub struct ContainerLogic<'a> { pub children: Children<'a>, pub scroll_offset: Signal<Vec2>, pub accessibility: Accessibility, pub attributes: Attributes }
pub struct ContainerView { pub core: ViewCore }
pub struct Container<'a> { pub id: String, pub logic: ContainerLogic<'a>, pub view: ContainerView }

impl<'a> Container<'a> {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: ContainerLogic { children: Children::new(), scroll_offset: Signal::new(Vec2::zero()), accessibility: Accessibility::default(), attributes: Attributes::default() }, view: ContainerView { core: ViewCore::new(style) } }
    }
    pub fn id(mut self, id: impl Into<String>) -> Self { self.id = id.into(); self }
    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self { self.logic.children.add(child); self.view.core.mark_dirty(); self }
}

impl<'a> Stylable for Container<'a> { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }

impl<'a> Component for Container<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.logic.children.get_all() }
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
        self.logic.children.layout_all(taffy, measurer, node); self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style_ref = self.view.core.style.borrow();
        let style = if is_group_hovered && style_ref.group_hover.is_some() { style_ref.group_hover.as_ref().unwrap() } else { &style_ref };
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style.rounding.nw); }
        let needs_clip = style.layout.overflow_x != Overflow::Visible || style.layout.overflow_y != Overflow::Visible;
        if needs_clip { renderer.push_clip(global_pos.x, global_pos.y, layout.size.width, layout.size.height); }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered || style.is_group, global_pos + self.logic.scroll_offset.get(), 0);
        if needs_clip { renderer.pop_clip(); }
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, delta: f32) { if self.view.core.style.borrow().layout.overflow_y == Overflow::Scroll { self.logic.scroll_offset.update(|o| o.y += delta); } }
    fn on_drag(&self, _: &mut UIEvent, delta: Vec2) { let style = self.view.core.style.borrow(); if style.layout.overflow_x == Overflow::Scroll || style.layout.overflow_y == Overflow::Scroll { self.logic.scroll_offset.update(|o| { o.x += delta.x; o.y += delta.y; }); } }
}
