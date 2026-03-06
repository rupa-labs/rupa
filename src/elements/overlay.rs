use crate::support::{Style, generate_id, Signal, Theme, Color, Accessibility, Attributes, Role, TextAlign, Vec2};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::elements::layout::container::Children;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- MODAL ---
pub struct ModalLogic<'a> { pub is_open: Signal<bool>, pub children: Children<'a>, pub header: Option<Box<dyn Component + 'a>>, pub footer: Option<Box<dyn Component + 'a>>, pub accessibility: Accessibility }
pub struct ModalView { pub core: ViewCore }
pub struct Modal<'a> { pub id: String, pub logic: ModalLogic<'a>, pub view: ModalView }
impl<'a> Modal<'a> {
    pub fn new(is_open: Signal<bool>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        style.background.color = Some(Color::Semantic("surface".into(), None));
        style.sizing.width = Some(400.0); style.padding = crate::support::Spacing::all(24.0);
        Self { id: generate_id(), logic: ModalLogic { is_open, children: Children::new(), header: None, footer: None, accessibility: Accessibility { role: Role::Alert, ..Default::default() } }, view: ModalView { core: ViewCore::new(style) } }
    }
    pub fn header(mut self, h: Box<dyn Component + 'a>) -> Self { self.logic.header = Some(h); self.view.core.mark_dirty(); self }
    pub fn child(mut self, c: Box<dyn Component + 'a>) -> Self { self.logic.children.add(c); self.view.core.mark_dirty(); self }
    pub fn footer(mut self, f: Box<dyn Component + 'a>) -> Self { self.logic.footer = Some(f); self.view.core.mark_dirty(); self }
}
impl<'a> Stylable for Modal<'a> { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl<'a> Component for Modal<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { let mut c = Vec::new(); if let Some(ref h) = self.logic.header { c.push(h.as_ref()); } c.extend(self.logic.children.get_all()); if let Some(ref f) = self.logic.footer { c.push(f.as_ref()); } c }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        if !self.logic.is_open.get() { return taffy.new_leaf(taffy::style::Style { display: taffy::style::Display::None, ..Default::default() }).unwrap(); }
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_with_children(self.view.core.get_style_mut().to_taffy(), &[]).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        let mut child_nodes = Vec::new(); if let Some(ref h) = self.logic.header { child_nodes.push(h.layout(taffy, measurer, Some(node))); }
        child_nodes.extend(self.logic.children.layout_all(taffy, measurer, node)); if let Some(ref f) = self.logic.footer { child_nodes.push(f.layout(taffy, measurer, Some(node))); }
        taffy.set_children(node, &child_nodes).unwrap(); self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, is_group_hovered: bool, global_pos: Vec2) {
        if !self.logic.is_open.get() { return; }
        let layout = taffy.layout(node).unwrap(); let style = self.view.core.style.borrow();
        if let Some(color) = style.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), 12.0); }
        let ch = taffy.children(node).unwrap(); let mut idx = 0;
        if let Some(ref h) = self.logic.header { if let Some(n) = ch.get(idx) { let cl = taffy.layout(*n).unwrap(); h.paint(renderer, taffy, *n, false, global_pos + Vec2::new(cl.location.x, cl.location.y)); idx += 1; } }
        self.logic.children.paint_all(renderer, taffy, node, is_group_hovered, global_pos, idx);
        if let Some(ref f) = self.logic.footer { if let Some(n) = ch.last() { let cl = taffy.layout(*n).unwrap(); f.paint(renderer, taffy, *n, false, global_pos + Vec2::new(cl.location.x, cl.location.y)); } }
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, ev: &mut UIEvent, d: f32) { if self.logic.is_open.get() { self.logic.children.list.iter().for_each(|c| c.on_scroll(ev, d)); } }
    fn on_drag(&self, ev: &mut UIEvent, d: Vec2) { if self.logic.is_open.get() { self.logic.children.list.iter().for_each(|c| c.on_drag(ev, d)); } }
}

// --- TOOLTIP ---
pub struct TooltipLogic { pub text: String }
pub struct TooltipView { pub core: ViewCore }
pub struct Tooltip { pub id: String, pub logic: TooltipLogic, pub view: TooltipView }
impl Tooltip {
    pub fn new(text: impl Into<String>) -> Self {
        Self { id: generate_id(), logic: TooltipLogic { text: text.into() }, view: TooltipView { core: ViewCore::default() } }
    }
}
impl Stylable for Tooltip { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Tooltip {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), self.view.core.get_style_mut().to_taffy()).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap(); self.view.core.set_node(SceneNode::from(new_node)); new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty(); node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.1, 0.1, 0.1, 0.9], 4.0);
        renderer.draw_text(&self.logic.text, global_pos.x + 4.0, global_pos.y + 2.0, 12.0, [1.0, 1.0, 1.0, 1.0], TextAlign::Left);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}
