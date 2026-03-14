use rupa_core::{Component, VNode, Id};
use rupa_vnode::{Style, Theme};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Component: Table 📊
pub struct Table<'a> {
    pub id: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<Box<dyn Component + 'a>>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            headers: vec![],
            rows: vec![],
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl<'a> Component for Table<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("table")
            .with_style(self.style.read().unwrap().clone())
            .with_key(self.id.clone())
    }
}

impl<'a> Stylable for Table<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
