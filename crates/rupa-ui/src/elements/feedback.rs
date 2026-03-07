use rupa_core::{Style, generate_id, Theme, Vec2};
use rupa_core::component::Component;
use rupa_core::view::ViewCore;
use rupa_core::renderer::{Renderer, TextMeasurer};
use crate::style::modifiers::base::Stylable;
use rupa_core::scene::SceneNode;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- PROGRESS ---
pub struct ProgressLogic { pub value: f32 }
pub struct ProgressView { pub core: ViewCore }
pub struct Progress { pub id: String, pub logic: ProgressLogic, pub view: ProgressView }
impl Progress {
    pub fn new(value: f32) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: ProgressLogic { value }, view: ProgressView { core: ViewCore::new(style) } }
    }
}
impl Stylable for Progress { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Progress {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style_ref = self.view.core.style.read().unwrap();
        if let Some(color) = style_ref.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw); }
    }
}

// --- SKELETON ---
pub struct SkeletonLogic {}
pub struct SkeletonView { pub core: ViewCore }
pub struct Skeleton { pub id: String, pub logic: SkeletonLogic, pub view: SkeletonView }
impl Skeleton {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: SkeletonLogic {}, view: SkeletonView { core: ViewCore::new(style) } }
    }
}
impl Stylable for Skeleton { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Skeleton {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style_ref = self.view.core.style.read().unwrap();
        if let Some(color) = style_ref.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw); }
    }
}

// --- BADGE ---
pub struct BadgeLogic { pub label: String }
pub struct BadgeView { pub core: ViewCore }
pub struct Badge { pub id: String, pub logic: BadgeLogic, pub view: BadgeView }
impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: BadgeLogic { label: label.into() }, view: BadgeView { core: ViewCore::new(style) } }
    }
}
impl Stylable for Badge { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Badge {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style_ref = self.view.core.style.read().unwrap();
        if let Some(color) = style_ref.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw); }
    }
}

// --- SPINNER ---
pub struct SpinnerLogic {}
pub struct SpinnerView { pub core: ViewCore }
pub struct Spinner { pub id: String, pub logic: SpinnerLogic, pub view: SpinnerView }
impl Spinner {
    pub fn new() -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: SpinnerLogic {}, view: SpinnerView { core: ViewCore::new(style) } }
    }
}
impl Stylable for Spinner { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Spinner {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style_ref = self.view.core.style.read().unwrap();
        if let Some(color) = style_ref.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw); }
    }
}

// --- ALERT ---
pub struct AlertLogic { pub title: String, pub content: String }
pub struct AlertView { pub core: ViewCore }
pub struct Alert { pub id: String, pub logic: AlertLogic, pub view: AlertView }
impl Alert {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        let mut style = Style::default(); Theme::current().apply_defaults(&mut style);
        Self { id: generate_id(), logic: AlertLogic { title: title.into(), content: content.into() }, view: AlertView { core: ViewCore::new(style) } }
    }
}
impl Stylable for Alert { fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.get_style_mut() } }
impl Component for Alert {
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
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap(); let style_ref = self.view.core.style.read().unwrap();
        if let Some(color) = style_ref.background.color.clone() { renderer.draw_rect(global_pos.x, global_pos.y, layout.size.width, layout.size.height, color.to_rgba(), style_ref.rounding.nw); }
    }
}
