use crate::utils::{Style, StyleModifier, generate_id, Signal, Variant};
use crate::Component;
use crate::container::Children;

/// A semantic component for lists of items.
pub struct ListGroup {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl ListGroup {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }
}

impl Component for ListGroup {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering ListGroup [{}]", self.id);
        self.children.render_all();
    }
}

/// A semantic component for toggling content visibility.
pub struct Collapse {
    pub id: String,
    pub is_open: Signal<bool>,
    pub style: Style,
    pub children: Children,
}

impl Collapse {
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

impl Component for Collapse {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.is_open.get() {
            log::debug!("Rendering Collapse [{}]", self.id);
            self.children.render_all();
        }
    }
}

/// A semantic component for contextual menus.
pub struct Dropdown {
    pub id: String,
    pub is_open: Signal<bool>,
    pub style: Style,
    pub children: Children,
}

impl Dropdown {
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

impl Component for Dropdown {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.is_open.get() {
            log::debug!("Rendering Dropdown [{}]", self.id);
            self.children.render_all();
        }
    }
}

// ... Keep existing Card and Accordion code ...
pub struct Card {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl Card {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
            children: Children::new(),
        }
    }
    pub fn style(mut self, modifier: impl StyleModifier) -> Self { modifier.apply(&mut self.style); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Card {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Card [{}]", self.id);
        self.children.render_all();
    }
}

pub struct Accordion {
    pub id: String,
    pub items: Vec<(String, Box<dyn Component>, Signal<bool>)>, // (title, content, is_open)
    pub style: Style,
}

impl Accordion {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            items: Vec::new(),
            style: Style::default(),
        }
    }
    pub fn item(mut self, title: impl Into<String>, content: Box<dyn Component>, open: Signal<bool>) -> Self {
        self.items.push((title.into(), content, open));
        self
    }
}

impl Component for Accordion {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Accordion [{}] with {} items", self.id, self.items.len());
    }
}
