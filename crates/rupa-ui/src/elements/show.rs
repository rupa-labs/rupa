use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Logic: Show 👁️
pub struct Show<'a> {
    pub id: String,
    pub condition: Signal<bool>,
    pub children: Vec<Box<dyn Component + 'a>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a> Show<'a> {
    pub fn new(condition: Signal<bool>) -> Self {
        Self {
            id: Id::next().to_string(),
            condition,
            children: Vec::new(),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn child(mut self, child: impl Component + 'a) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a> Component for Show<'a> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { self.children.iter().map(|c| c.as_ref()).collect() }
    
    fn render(&self) -> VNode {
        if self.condition.get() {
            VNode::fragment(self.children.iter().map(|c| c.render()).collect())
        } else {
            VNode::Empty
        }
    }
}

impl<'a> Stylable for Show<'a> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
