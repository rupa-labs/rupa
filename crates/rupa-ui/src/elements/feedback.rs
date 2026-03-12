use rupa_core::{Component, VNode, ViewCore, Id, Signal};
use rupa_vnode::{Style, Variant};
use crate::style::modifiers::Stylable;
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
            id: Id::next().to_string(),
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
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("progress")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_attr("value", self.value.get().to_string())
            .with_attr("max", self.max.to_string())
    }
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
        Self { id: Id::next().to_string(), view: Arc::new(ViewCore::new()) }
    }
}

impl Component for Spinner {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("spinner")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
    }
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
            id: Id::next().to_string(),
            message: message.into(),
            variant,
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for Alert {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("alert")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text(self.message.clone()))
    }
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
            id: Id::next().to_string(),
            text: text.into(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for Badge {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("badge")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
            .with_child(VNode::text(self.text.clone()))
    }
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
        Self { id: Id::next().to_string(), view: Arc::new(ViewCore::new()) }
    }
}

impl Component for Skeleton {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    fn render(&self) -> VNode {
        VNode::element("skeleton")
            .with_key(self.id.clone())
            .with_style(self.view.style.read().unwrap().clone())
    }
}

impl Stylable for Skeleton {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
