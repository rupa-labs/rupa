use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style, Theme};
use std::sync::{Arc, RwLock};

/// A terminal-specific progress bar.
pub struct Progress {
    pub id: String,
    pub value: Signal<f32>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Progress {
    pub fn new(value: Signal<f32>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            value,
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn id(&self) -> &str { &self.id }
    pub fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
}

impl Component for Progress {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("progress")
            .with_style(self.style.read().unwrap().clone())
            .with_attr("value", self.value.get().to_string())
            .with_key(self.id.clone())
    }
}
