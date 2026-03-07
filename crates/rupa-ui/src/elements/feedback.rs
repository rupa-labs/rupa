use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_styling::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::RwLockWriteGuard;

// --- PROGRESS ---

pub struct ProgressLogic {
    pub value: f32, // 0.0 to 1.0
}

pub struct ProgressView {
    pub core: ViewCore,
}

pub struct Progress {
    pub id: String,
    pub logic: ProgressLogic,
    pub view: ProgressView,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: generate_id(),
            logic: ProgressLogic { value },
            view: ProgressView { core: view },
        }
    }
}

impl Component for Progress {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "progress".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("value", self.logic.value.to_string());
                attr
            },
            children: vec![],
            key: Some(self.id.clone()),
        })
    }


    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.view.core.clear_dirty();
        node
    }

    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Progress {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- SKELETON ---

pub struct SkeletonLogic {}
pub struct SkeletonView { pub core: ViewCore }
pub struct Skeleton { pub id: String, pub logic: SkeletonLogic, pub view: SkeletonView }

impl Skeleton {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self { id: generate_id(), logic: SkeletonLogic {}, view: SkeletonView { core: view } }
    }
}

impl Component for Skeleton {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "skeleton".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Skeleton {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- BADGE ---

pub struct BadgeLogic { pub label: String }
pub struct BadgeView { pub core: ViewCore }
pub struct Badge { pub id: String, pub logic: BadgeLogic, pub view: BadgeView }

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self { id: generate_id(), logic: BadgeLogic { label: label.into() }, view: BadgeView { core: view } }
    }
}

impl Component for Badge {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "badge".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: vec![VNode::text(self.logic.label.clone())],
            key: Some(self.id.clone()),
        })
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Badge {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- SPINNER ---

pub struct SpinnerLogic {}
pub struct SpinnerView { pub core: ViewCore }
pub struct Spinner { pub id: String, pub logic: SpinnerLogic, pub view: SpinnerView }

impl Spinner {
    pub fn new() -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self { id: generate_id(), logic: SpinnerLogic {}, view: SpinnerView { core: view } }
    }
}

impl Component for Spinner {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "spinner".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Spinner {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}

// --- ALERT ---

pub struct AlertLogic { pub title: String, pub content: String }
pub struct AlertView { pub core: ViewCore }
pub struct Alert { pub id: String, pub logic: AlertLogic, pub view: AlertView }

impl Alert {
    pub fn new(title: impl Into<String>, content: impl Into<String>) -> Self {
        let view = ViewCore::new();
        Theme::current().apply_defaults(&mut view.style());
        Self { id: generate_id(), logic: AlertLogic { title: title.into(), content: content.into() }, view: AlertView { core: view } }
    }
}

impl Component for Alert {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn render(&self) -> VNode {
        VNode::Element(VElement {
            tag: "alert".to_string(),
            style: self.view.core.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("title", self.logic.title.clone());
                attr
            },
            children: vec![VNode::text(self.logic.content.clone())],
            key: Some(self.id.clone()),
        })
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.core.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.core.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.core.is_dirty() }
    fn mark_dirty(&self) { self.view.core.mark_dirty(); }
    fn clear_dirty(&self) { self.view.core.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, parent: Option<NodeId>) -> NodeId {
        let style = self.view.core.style.read().unwrap().to_taffy();
        let node = if let Some(existing) = self.view.core.get_node() {
            if self.view.core.is_dirty() { taffy.set_style(existing.raw(), style).unwrap(); }
            existing.raw()
        } else {
            let new_node = taffy.new_leaf(style).unwrap();
            self.view.core.set_node(SceneNode::from(new_node));
            new_node
        };
        if let Some(p) = parent {
            let cur = taffy.children(p).unwrap_or_default();
            if !cur.contains(&node) { taffy.add_child(p, node).unwrap(); }
        }
        self.view.core.clear_dirty();
        node
    }
    fn paint(&self, renderer: &mut dyn Renderer, taffy: &TaffyTree<()>, node: NodeId, _is_group_hovered: bool, global_pos: Vec2) {
        let layout = taffy.layout(node).unwrap();
        let style_ref = self.view.core.style.read().unwrap();
        if let Some(ref color) = style_ref.background.color {
            renderer.draw_rect(
                global_pos.x + layout.location.x,
                global_pos.y + layout.location.y,
                layout.size.width,
                layout.size.height,
                color.to_rgba(),
                style_ref.rounding.nw
            );
        }
    }
}

impl Stylable for Alert {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.core.style() }
}
