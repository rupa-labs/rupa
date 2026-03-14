use rupa_core::{Component, VNode, Id, Signal};
use rupa_vnode::{Style, Theme, TextAlign};
use crate::style::modifiers::Stylable;
use std::sync::{RwLockWriteGuard, Arc, RwLock};

// --- LABEL ---

/// A semantic label for form elements.
pub struct Label {
    pub id: String,
    pub text: String,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Label {
    pub fn new(text: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            text: text.into(),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Label {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("label")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("text", self.text.clone())
            .with_child(VNode::text(self.text.clone()))
            .with_key(self.id.clone())
    }
}

impl Stylable for Label {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

// --- INPUT ---

/// A single-line text input field.
pub struct Input {
    pub id: String,
    pub value: Signal<String>,
    pub label: Option<String>,
    pub placeholder: String,
    pub on_input: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub on_submit: Option<Arc<dyn Fn(String) + Send + Sync>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Input {
    pub fn new(placeholder: impl Into<String>) -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        // Inputs typically have borders in TUI
        style.border.width = 1.0;
        
        Self {
            id: Id::next().to_string(),
            value: Signal::new(String::new()),
            label: None,
            placeholder: placeholder.into(),
            on_input: None,
            on_submit: None,
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn value(mut self, value: Signal<String>) -> Self {
        self.value = value;
        self
    }

    pub fn on_input(mut self, f: impl Fn(String) + Send + Sync + 'static) -> Self {
        self.on_input = Some(Arc::new(f));
        self
    }

    pub fn on_submit(mut self, f: impl Fn(String) + Send + Sync + 'static) -> Self {
        self.on_submit = Some(Arc::new(f));
        self
    }

    pub fn with_key(mut self, key: impl Into<String>) -> Self {
        self.id = key.into();
        self
    }
}

impl Component for Input {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        let value_signal = self.value.clone();
        let on_input_handler = self.on_input.clone();
        let on_submit_handler = self.on_submit.clone();

        let mut node = VNode::element("input")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("placeholder", self.placeholder.clone())
            .with_attr("value", self.value.get())
            .with_key(self.id.clone())
            .with_label_align(TextAlign::Left);

        if let Some(ref l) = self.label {
            node = node.with_label(l.clone());
        }

        // Always sync signal on input
        node = node.on_input(move |v| {
            value_signal.set(v.clone());
            if let Some(ref h) = on_input_handler {
                h(v);
            }
        });

        if let Some(ref h) = on_submit_handler {
            node = node.on_submit({
                let h = h.clone();
                move |v| h(v)
            });
        }

        node
    }
}

impl Stylable for Input {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

// --- CHECKBOX ---

/// A classic binary selection toggle.
pub struct Checkbox {
    pub id: String,
    pub checked: Signal<bool>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Checkbox {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            checked: Signal::new(false),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Checkbox {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("checkbox")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("checked", self.checked.get().to_string())
            .with_key(self.id.clone())
    }
}

impl Stylable for Checkbox {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

// --- SWITCH ---

/// A modern binary toggle switch.
pub struct Switch {
    pub id: String,
    pub active: Signal<bool>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Switch {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            active: Signal::new(false),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Switch {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("switch")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("active", self.active.get().to_string())
            .with_key(self.id.clone())
    }
}

impl Stylable for Switch {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

// --- RADIO ---

/// A radio button for mutually exclusive selection.
pub struct Radio {
    pub id: String,
    pub selected: Signal<bool>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Radio {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            selected: Signal::new(false),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Radio {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("radio")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("selected", self.selected.get().to_string())
            .with_key(self.id.clone())
    }
}

impl Stylable for Radio {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}

// --- SELECT ---

/// A dropdown selection menu.
pub struct Select {
    pub id: String,
    pub options: Vec<String>,
    pub selected_index: Signal<Option<usize>>,
    pub style: Arc<RwLock<Style>>,
    pub prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl Select {
    pub fn new() -> Self {
        let mut style = Style::default();
        Theme::current().apply_defaults(&mut style);
        Self {
            id: Id::next().to_string(),
            options: vec![],
            selected_index: Signal::new(None),
            style: Arc::new(RwLock::new(style)),
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for Select {
    fn id(&self) -> &str { &self.id }
    fn get_style(&self) -> Arc<RwLock<Style>> { self.style.clone() }
    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> { self.prev_vnode.clone() }
    
    fn render(&self) -> VNode {
        VNode::element("select")
            .with_style(self.get_style().read().unwrap().clone())
            .with_attr("options_count", self.options.len().to_string())
            .with_key(self.id.clone())
    }
}

impl Stylable for Select {
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> { self.style.write().unwrap() }
}
