use rupa_core::{Component, VNode, VElement, ViewCore, Id, Signal};
use rupa_vnode::{Style, Theme, Attributes};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc};

// --- LABEL ---

/// A semantic label for form elements.
pub struct Label {
    pub id: String,
    pub text: String,
    pub view: Arc<ViewCore>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            text: text.into(),
            view,
        }
    }
}

impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "label".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("text", self.text.clone());
                attr
            },
            motion: None,
            children: vec![VNode::text(self.text.clone())],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Label {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- INPUT ---

/// A single-line text input field.
pub struct Input {
    pub id: String,
    pub value: Signal<String>,
    pub placeholder: String,
    pub view: Arc<ViewCore>,
}

impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            value: Signal::new(String::new()),
            placeholder: placeholder.into(),
            view,
        }
    }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "input".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("placeholder", self.placeholder.clone());
                attr.insert("value", self.value.get());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Input {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- CHECKBOX ---

/// A classic binary selection toggle.
pub struct Checkbox {
    pub id: String,
    pub checked: Signal<bool>,
    pub view: Arc<ViewCore>,
}

impl Checkbox {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            checked: Signal::new(false),
            view,
        }
    }
}

impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "checkbox".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("checked", self.checked.get().to_string());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Checkbox {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- SWITCH ---

/// A modern binary toggle switch.
pub struct Switch {
    pub id: String,
    pub active: Signal<bool>,
    pub view: Arc<ViewCore>,
}

impl Switch {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            active: Signal::new(false),
            view,
        }
    }
}

impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "switch".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("active", self.active.get().to_string());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Switch {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- RADIO ---

/// A radio button for mutually exclusive selection.
pub struct Radio {
    pub id: String,
    pub selected: Signal<bool>,
    pub view: Arc<ViewCore>,
}

impl Radio {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            selected: Signal::new(false),
            view,
        }
    }
}

impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "radio".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("selected", self.selected.get().to_string());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Radio {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}

// --- SELECT ---

/// A dropdown selection menu.
pub struct Select {
    pub id: String,
    pub options: Vec<String>,
    pub selected_index: Signal<Option<usize>>,
    pub view: Arc<ViewCore>,
}

impl Select {
    pub fn new() -> Self {
        let view = Arc::new(ViewCore::new());
        Theme::current().apply_defaults(&mut view.style());
        Self {
            id: Id::next().to_string(),
            options: vec![],
            selected_index: Signal::new(None),
            view,
        }
    }
}

impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn view_core(&self) -> Arc<ViewCore> { self.view.clone() }
    
    fn render(&self) -> VNode {
        VNode::Element(VElement { 
            handlers: Default::default(), 
            tag: "select".to_string(),
            style: self.view.style.read().unwrap().clone(),
            attributes: {
                let mut attr = Attributes::new();
                attr.insert("options_count", self.options.len().to_string());
                attr
            },
            motion: None,
            children: vec![],
            key: Some(self.id.clone()),
        })
    }
}

impl Stylable for Select {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.view.style() }
}
