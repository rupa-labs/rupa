use crate::utils::{Style, StyleModifier, generate_id};
use crate::Component;
use crate::container::Children;
use crate::elements::Text;

/// A generic container for UI layout.
pub struct Div {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl Div {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
            children: Children::new(),
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }

    /// Flexible styling using the modular StyleModifier API.
    /// Supports Style objects, tuples of modifiers, or closures.
    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self {
        self.children.add(child);
        self
    }

    pub fn children(mut self, children: Vec<Box<dyn Component>>) -> Self {
        self.children.append(children);
        self
    }

    pub fn text(mut self, content: impl Into<String>) -> Self {
        self.children.add(Box::new(Text::new(content)));
        self
    }
}

impl Component for Div {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Div [ID: {}] with {} children", self.id, self.children.list.len());
        self.children.render_all();
    }
}
