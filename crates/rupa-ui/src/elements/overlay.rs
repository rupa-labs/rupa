use rupa_core::{Component, VNode, VElement, ViewCore, Id};
use rupa_vnode::{Style, Attributes};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc};

// --- MODAL ---

pub struct Modal<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Modal<'a> {
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

impl<'a> Component for Modal<'a> {
    fn id(&self) -> &str { &self.id }
    fn is_modal(&self) -> bool { true }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "modal".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Modal<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- TOOLTIP ---

pub struct Tooltip<'a> {
    pub id: String,
    pub text: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Tooltip<'a> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            id: Id::next().to_string(),
            text: text.into(),
            children: Children::new(),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Tooltip<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "tooltip".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("text", self.text.clone());
                attr
            },
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}
