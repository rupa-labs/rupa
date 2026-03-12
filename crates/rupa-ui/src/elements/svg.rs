use rupa_core::{Component, VNode, VElement, ViewCore, Id};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

// --- SVG ---

pub struct Svg {
    pub id: String,
    pub path: String,
    pub view: Arc<ViewCore>,
}

impl Svg {
    pub fn new(path: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            path: path.into(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for Svg {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "svg".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("path", self.path.clone());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Svg {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- ICON ---

pub struct Icon {
    pub id: String,
    pub name: String,
    pub view: Arc<ViewCore>,
}

impl Icon {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            name: name.into(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl Component for Icon {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "icon".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("name", self.name.clone());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}
