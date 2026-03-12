use rupa_core::{Component, VNode, VElement, ViewCore, Id};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::base::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

// --- BRAND ---

/// A specialized wrapper for brand identities.
pub struct Brand {
    pub id: String,
    pub name: String,
    pub view: Arc<ViewCore>,
}

impl Brand {
    pub fn new(name: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            name: name.into(),
            view,
        }
    }
}

impl Stylable for Brand {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

impl Component for Brand {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "brand".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("name", self.name.clone());
                attr
            },
            motion: None,
            children: vec![VNode::text(self.name.clone())],
            key: Some(self.id.clone()),
        })
    }
}
