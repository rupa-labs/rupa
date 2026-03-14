use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// # Logic: ForEach 🔁
pub struct ForEach<'a, T: Clone + Send + Sync + 'static> {
    pub id: String,
    pub items: Signal<Vec<T>>,
    pub template: Arc<dyn Fn(&T) -> Box<dyn Component + 'a> + Send + Sync>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl<'a, T: Clone + Send + Sync + 'static> ForEach<'a, T> {
    pub fn new<F>(items: Signal<Vec<T>>, template: F) -> Self 
    where F: Fn(&T) -> Box<dyn Component + 'a> + Send + Sync + 'static {
        Self {
            id: Id::next().to_string(),
            items,
            template: Arc::new(template),
            style: Arc::new(RwLock::new(Style::default())),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl<'a, T: Clone + Send + Sync + 'static> Component for ForEach<'a, T> {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        let items = self.items.get();
        let nodes = items.iter()
            .map(|item| (self.template)(item).render())
            .collect();
        VNode::fragment(nodes)
    }
}

impl<'a, T: Clone + Send + Sync + 'static> Stylable for ForEach<'a, T> {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
