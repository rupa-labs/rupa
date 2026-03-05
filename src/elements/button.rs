use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant};
use crate::Component;
use crate::container::Children;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// A semantic Button component with reactive states.
pub struct Button {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub size: ButtonSize,
    pub is_disabled: Signal<bool>,
    pub is_loading: Signal<bool>,
    pub style: Style,
    pub accessibility: Accessibility,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            label: label.into(),
            variant: Variant::Primary,
            size: ButtonSize::Md,
            is_disabled: Signal::new(false),
            is_loading: Signal::new(false),
            style: Style::default(),
            accessibility: Accessibility { 
                role: Role::Button, 
                ..Default::default() 
            },
        }
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
    pub fn size(mut self, s: ButtonSize) -> Self { self.size = s; self }
    pub fn disabled(self, signal: Signal<bool>) -> Self { let mut s = self; s.is_disabled = signal; s }
    pub fn loading(self, signal: Signal<bool>) -> Self { let mut s = self; s.is_loading = signal; s }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Button {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!(
            "Rendering Button [{}] '{}' variant={:?} disabled={}", 
            self.id, self.label, self.variant, self.is_disabled.get()
        );
    }
}

/// A specialized small button for closing overlays or alerts.
pub struct CloseButton {
    pub id: String,
    pub is_disabled: Signal<bool>,
    pub style: Style,
}

impl CloseButton {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            is_disabled: Signal::new(false),
            style: Style::default(),
        }
    }
}

impl Component for CloseButton {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering CloseButton [{}]", self.id);
    }
}

/// A container for grouping multiple buttons together.
pub struct ButtonGroup {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl ButtonGroup {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = crate::utils::Display::Flex;
        Self {
            id: generate_id(),
            style,
            children: Children::new(),
        }
    }

    pub fn child(mut self, button: Button) -> Self {
        self.children.add(Box::new(button));
        self
    }
}

impl Component for ButtonGroup {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering ButtonGroup [{}]", self.id);
        self.children.render_all();
    }
}
