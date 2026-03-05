use crate::utils::{Style, StyleModifier, generate_id, Accessibility, Role, Signal, Variant};
use crate::Component;
use crate::container::Children;

/// A visual indicator for process completion.
pub struct Progress {
    pub id: String,
    pub value: Signal<f32>, // 0.0 to 100.0
    pub variant: Variant,
    pub style: Style,
}

impl Progress {
    pub fn new(value_signal: Signal<f32>) -> Self {
        Self {
            id: generate_id(),
            value: value_signal,
            variant: Variant::Primary,
            style: Style::default(),
        }
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
}

impl Component for Progress {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Progress [{}] val={}%", self.id, self.value.get());
    }
}

/// A loading animation.
pub struct Spinner {
    pub id: String,
    pub variant: Variant,
    pub style: Style,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            variant: Variant::Primary,
            style: Style::default(),
        }
    }

    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
}

impl Component for Spinner {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Spinner [{}]", self.id);
    }
}

/// A skeleton loader component for content that is still loading.
pub struct Placeholder {
    pub id: String,
    pub style: Style,
}

impl Placeholder {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            style: Style::default(),
        }
    }
}

impl Component for Placeholder {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Placeholder [{}]", self.id);
    }
}

// ... Keep existing Alert and Badge code ...
pub struct Alert {
    pub id: String,
    pub variant: Variant,
    pub is_dismissible: bool,
    pub is_visible: Signal<bool>,
    pub style: Style,
    pub children: Children,
}

impl Alert {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            variant: Variant::Info,
            is_dismissible: false,
            is_visible: Signal::new(true),
            style: Style::default(),
            children: Children::new(),
        }
    }
    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
    pub fn dismissible(mut self) -> Self { self.is_dismissible = true; self }
    pub fn visible(self, signal: Signal<bool>) -> Self { let mut s = self; s.is_visible = signal; s }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Alert {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        if self.is_visible.get() {
            log::debug!("Rendering Alert [{}] variant={:?}", self.id, self.variant);
            self.children.render_all();
        }
    }
}

pub struct Badge {
    pub id: String,
    pub label: String,
    pub variant: Variant,
    pub is_pill: bool,
    pub style: Style,
}

impl Badge {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: generate_id(),
            label: label.into(),
            variant: Variant::Secondary,
            is_pill: false,
            style: Style::default(),
        }
    }
    pub fn variant(mut self, v: Variant) -> Self { self.variant = v; self }
    pub fn pill(mut self) -> Self { self.is_pill = true; self }
}

impl Component for Badge {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Badge [{}] '{}' variant={:?}", self.id, self.label, self.variant);
    }
}
