use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style, Theme};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Component: Select 🔽
pub struct Select {
    pub id: String,
    pub options: Vec<String>,
    pub selected_index: Signal<Option<usize>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Select {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            options: vec![],
            selected_index: Signal::new(None),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Select {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn render(&self) -> VNode {
        VNode::element("select")
            .with_style(self.style.read().unwrap().clone())
            .with_attr("options_count", self.options.len().to_string())
            .with_key(self.id.clone())
    }
}

impl Stylable for Select {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
