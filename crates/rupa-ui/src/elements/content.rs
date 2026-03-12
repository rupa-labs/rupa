use rupa_core::{Component, VNode, VElement, ViewCore, Id, Signal};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::Stylable;
use crate::elements::Children;
use std::sync::{RwLockWriteGuard, Arc};

// --- CARD ---

/// A container component with an elevated surface.
pub struct Card<'a> {
    pub id: String,
    pub children: Children<'a>,
    pub view: Arc<ViewCore>,
}

impl<'a> Card<'a> {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            children: Children::new(),
            view,
        }
    }

    pub fn child(mut self, child: Box<dyn Component + 'a>) -> Self {
        self.children.push(child);
        self.view.mark_dirty();
        self
    }
}

impl<'a> Component for Card<'a> {
    fn id(&self) -> &str { &self.id }
    fn children(&self) -> Vec<&dyn Component> { self.children.as_refs() }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "card".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: self.children.render_all(),
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Card<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- TABLE ---

/// A component for displaying structured data.
pub struct Table<'a> {
    pub id: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<Box<dyn Component + 'a>>>,
    pub view: Arc<ViewCore>,
}

impl<'a> Table<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            headers: vec![],
            rows: vec![],
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Table<'a> {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "table".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Table<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- ACCORDION ---

/// A space-saving collapsible content component.
pub struct Accordion<'a> {
    pub id: String,
    pub items: Vec<(String, Box<dyn Component + 'a>)>,
    pub expanded_index: Signal<Option<usize>>,
    pub view: Arc<ViewCore>,
}

impl<'a> Accordion<'a> {
    pub fn new() -> Self {
        Self {
            id: Id::next().to_string(),
            items: vec![],
            expanded_index: Signal::new(None),
            view: Arc::new(ViewCore::new()),
        }
    }
}

impl<'a> Component for Accordion<'a> {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "accordion".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: Attributes::default(),
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl<'a> Stylable for Accordion<'a> {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
