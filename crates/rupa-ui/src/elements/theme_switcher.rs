use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// A utility component to toggle between Light and Dark modes.
pub struct ThemeSwitcher {
    pub id: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl ThemeSwitcher {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for ThemeSwitcher {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("theme-switcher")
            .with_style(self.get_style().read().unwrap().clone())
            .with_key(self.id.clone())
    }
}

impl Stylable for ThemeSwitcher {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
