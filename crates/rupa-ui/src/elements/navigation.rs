use rupa_core::{Component, VNode, VElement, ViewCore, Id};
use rupa_vnode::{Style, Attributes};

use crate::style::modifiers::base::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc};

// --- NAVBAR ---

pub struct Navbar<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Navbar<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            view: Arc::new(ViewCore::new()),
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.children.push(child);
        self.view.mark_dirty();
        self
    }
}

impl<'a> Component for Navbar<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "navbar".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Navbar<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- TABS ---

pub struct Tabs<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Tabs<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Tabs<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "tabs".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}

// --- BREADCRUMB ---

pub struct Breadcrumb<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Breadcrumb<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Breadcrumb<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "breadcrumb".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}
