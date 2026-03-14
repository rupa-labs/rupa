use rupa_core::{Component, VNode, Id};
use rupa_vnode::Style;
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A component that expands to fill available space.
pub struct Spacer {
    pub id: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Spacer {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.flex.flex_grow = 1.0;
        
        Self {
            id: Id::next().to_string(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Spacer {
    fn id(&self) -> &str { &self.id }
    fn style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("spacer")
            .with_style(self.get_style().read().unwrap().clone())
            .with_key(self.id.clone())
    }
}

impl Stylable for Spacer {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
