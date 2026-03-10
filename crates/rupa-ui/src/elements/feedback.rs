use rupa_core::{Component, VNode, Vec2, ViewCore, generate_id, Signal, Renderer, TextMeasurer, SceneNode};
use rupa_vnode::{Style, Variant};
use crate::style::modifiers::base::Stylable;
use taffy::prelude::*;
use std::sync::{RwLockWriteGuard, Arc};

// --- PROGRESS ---

/// A bar showing completion status of a task.
pub struct Progress {
    pub id: String,
    pub value: Signal<f32>,
    pub max: f32,
    pub view: Arc<ViewCore>,
}

impl Progress {
    pub fn new(value: Signal<f32>) -> Self {
        Self {
            id: generate_id(),
            value,
            max: 100.0,
            view: Arc::new(ViewCore::new()),
        }
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }
}

impl Component for Progress {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("progress")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
    }

    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }

    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }

    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl Stylable for Progress {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- SPINNER ---

/// An animated indicator for loading states.
pub struct Spinner {
    pub id: String,
    pub view: Arc<ViewCore>,
}

impl Spinner {
    pub fn new() -> Self {
        Self { id: generate_id(), view: Arc::new(ViewCore::new()) }
    }
}

impl Component for Spinner {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("spinner")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl Stylable for Spinner {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- ALERT ---

/// A notification banner for important messages.
pub struct Alert {
    pub id: String,
    pub message: String,
    pub variant: Variant,
    pub view: Arc<ViewCore>,
}

impl Alert {
    pub fn new(message: impl Into<String>, variant: Variant) -> Self {
        Self {
            id: generate_id(),
            message: message.into(),
            variant,
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for Alert {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("alert")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text(self.message.clone()))
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl Stylable for Alert {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- BADGE ---

/// A small visual indicator for status or counts.
pub struct Badge {
    pub id: String,
    pub text: String,
    pub view: Arc<ViewCore>,
}

impl Badge {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            text: text.into(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for Badge {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("badge")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text(self.text.clone()))
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl Stylable for Badge {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- SKELETON ---

/// A visual placeholder for loading content.
pub struct Skeleton {
    pub id: String,
    pub view: Arc<ViewCore>,
}

impl Skeleton {
    pub fn new() -> Self {
        Self { id: generate_id(), view: Arc::new(ViewCore::new()) }
    }
}

impl Component for Skeleton {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { vec![] }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("skeleton")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
    }
    fn get_node(&self) -> Option<SceneNode> { self.view.get_node() }
    fn set_node(&self, node: SceneNode) { self.view.set_node(node); }
    fn is_dirty(&self) -> bool { self.view.is_dirty() }
    fn mark_dirty(&self) { self.view.mark_dirty(); }
    fn clear_dirty(&self) { self.view.clear_dirty(); }
    fn layout(&self, taffy: &mut TaffyTree<()>, _measurer: &dyn TextMeasurer, _parent: Option<NodeId>) -> NodeId {
        let node = taffy.new_leaf(self.view.style().to_taffy()).unwrap();
        self.view.set_node(SceneNode::from(node));
        self.view.clear_dirty();
        node
    }
    fn paint(&self, _renderer: &mut dyn Renderer, _taffy: &TaffyTree<()>, _node: NodeId, _is_group_hovered: bool, _global_pos: Vec2) {}
}

impl Stylable for Skeleton {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
