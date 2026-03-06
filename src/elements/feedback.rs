use crate::support::{Style, generate_id, Signal, Theme, Color, Accessibility, Attributes, Vec2, Variant};
use crate::core::component::Component;
use crate::core::ViewCore;
use crate::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::utilities::Stylable;
use crate::platform::events::UIEvent;
use crate::scene::SceneNode;
use taffy::prelude::*;
use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};

// --- SPINNER ---
pub struct SpinnerLogic { pub is_animating: Signal<bool> }
pub struct SpinnerView { pub core: ViewCore }
pub struct Spinner { pub id: String, pub logic: SpinnerLogic, pub view: SpinnerView }
impl Spinner {
    pub fn new() -> Self {
        Self { id: generate_id(), logic: SpinnerLogic { is_animating: Signal::new(true) }, view: SpinnerView { core: ViewCore::default() } }
    }
}
impl Stylable for Spinner { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Spinner {
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
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.5, 0.5, 0.5, 1.0], layout.size.width / 2.0);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- PROGRESS ---
pub struct ProgressLogic { pub value: Signal<f32> }
pub struct ProgressView { pub core: ViewCore }
pub struct Progress { pub id: String, pub logic: ProgressLogic, pub view: ProgressView }
impl Progress {
    pub fn new(value: Signal<f32>) -> Self {
        Self { id: generate_id(), logic: ProgressLogic { value }, view: ProgressView { core: ViewCore::default() } }
    }
}
impl Stylable for Progress { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Progress {
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
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.2, 0.2, 0.2, 1.0], 4.0);
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width * self.logic.value.get(), layout.size.height, [0.0, 0.5, 1.0, 1.0], 4.0);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- BADGE ---
pub struct BadgeLogic { pub label: String }
pub struct BadgeView { pub core: ViewCore }
pub struct Badge { pub id: String, pub logic: BadgeLogic, pub view: BadgeView }
impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self { id: generate_id(), logic: BadgeLogic { label: label.into() }, view: BadgeView { core: ViewCore::default() } }
    }
}
impl Stylable for Badge { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Badge {
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
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.0, 0.7, 0.3, 1.0], 10.0);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- SKELETON ---
pub struct SkeletonLogic { pub is_loading: Signal<bool> }
pub struct SkeletonView { pub core: ViewCore }
pub struct Skeleton { pub id: String, pub logic: SkeletonLogic, pub view: SkeletonView }
impl Skeleton {
    pub fn new() -> Self {
        Self { id: generate_id(), logic: SkeletonLogic { is_loading: Signal::new(true) }, view: SkeletonView { core: ViewCore::default() } }
    }
}
impl Stylable for Skeleton { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Skeleton {
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
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.3, 0.3, 0.3, 1.0], 4.0);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}

// --- ALERT ---
pub struct AlertLogic { pub message: String, pub variant: Variant }
pub struct AlertView { pub core: ViewCore }
pub struct Alert { pub id: String, pub logic: AlertLogic, pub view: AlertView }
impl Alert {
    pub fn new(message: impl Into<String>) -> Self {
        Self { id: generate_id(), logic: AlertLogic { message: message.into(), variant: Variant::Danger }, view: AlertView { core: ViewCore::default() } }
    }
}
impl Stylable for Alert { fn get_style_mut(&self) -> RefMut<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Alert {
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
            let new_node = taffy.new_leaf(self.view.core.get_style_mut().to_taffy()).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent { let cur = taffy.children(p).unwrap_or_default(); if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); } }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, [0.8, 0.2, 0.2, 1.0], 4.0);
    }
    fn on_click(&self, _: &mut UIEvent) {}
    fn on_scroll(&self, _: &mut UIEvent, _: f32) {}
    fn on_drag(&self, _: &mut UIEvent, _: Vec2) {}
}
