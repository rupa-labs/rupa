use crate::utils::{Style, StyleModifier, generate_id, Signal, Variant};
use crate::Component;
use crate::container::Children;

/// A high-priority overlay that blocks interaction with the rest of the UI.
pub struct Modal {
    pub id: String,
    pub is_open: Signal<bool>,
    pub style: Style,
    pub children: Children,
}

impl Modal {
    pub fn new(open_signal: Signal<bool>) -> Self {
        Self {
            id: generate_id(),
            is_open: open_signal,
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Modal {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.is_open.get() {
            log::debug!("Rendering Modal [{}]", self.id);
            self.children.render_all();
        }
    }
}

/// A sidebar overlay that slides from the edge of the screen.
pub struct Offcanvas {
    pub id: String,
    pub is_open: Signal<bool>,
    pub style: Style,
    pub children: Children,
}

impl Offcanvas {
    pub fn new(open_signal: Signal<bool>) -> Self {
        Self {
            id: generate_id(),
            is_open: open_signal,
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for Offcanvas {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.is_open.get() {
            log::debug!("Rendering Offcanvas [{}]", self.id);
            self.children.render_all();
        }
    }
}

/// A lightweight notification that disappears automatically.
pub struct Toast {
    pub id: String,
    pub message: String,
    pub variant: Variant,
    pub is_visible: Signal<bool>,
    pub style: Style,
}

impl Toast {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            message: message.into(),
            variant: Variant::Primary,
            is_visible: Signal::new(true),
            style: Style::default(),
        }
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
}

impl Component for Toast {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.is_visible.get() {
            log::debug!("Rendering Toast [{}]: {}", self.id, self.message);
        }
    }
}
