use rupa_core::{Component, VNode, Id};
use rupa_vnode::Style;
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

/// The root body element of a Rupa application.
pub struct Body {
    pub id: String,
    pub root: Box<dyn Component>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Body {
    pub fn new(root: impl Component + 'static) -> Self {
        let mut style = Style::default();
        rupa_vnode::style::theme::Theme::current().apply_defaults(&mut style);
        
        Self {
            id: Id::next().to_string(),
            root: Box::new(root),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Body {
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    fn children(&self) -> Vec<&dyn Component> { vec![self.root.as_ref()] }
    
    fn render(&self) -> VNode {
        VNode::element("body")
            .with_style(self.style.read().unwrap().clone())
            .with_child(self.root.render())
            .with_key(self.id.clone())
    }
}

impl Stylable for Body {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
