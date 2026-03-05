use crate::utils::{Style, StyleModifier, generate_id, FlexDirection, Display, AlignItems, JustifyContent};
use crate::Component;
use crate::container::Children;

/// A vertical stack layout helper.
pub struct VStack {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl VStack {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = Display::Flex;
        style.flex.direction = FlexDirection::Column;
        Self { id: generate_id(), style, children: Children::new() }
    }

    pub fn gap(mut self, val: f32) -> Self { self.style.grid.gap = (val, val); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for VStack {
    fn id(&self) -> &str { &self.id }
    fn render(&self) { self.children.render_all(); }
}

/// A horizontal stack layout helper.
pub struct HStack {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl HStack {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.layout.display = Display::Flex;
        style.flex.direction = FlexDirection::Row;
        style.flex.align_items = AlignItems::Center;
        Self { id: generate_id(), style, children: Children::new() }
    }

    pub fn gap(mut self, val: f32) -> Self { self.style.grid.gap = (val, val); self }
    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for HStack {
    fn id(&self) -> &str { &self.id }
    fn render(&self) { self.children.render_all(); }
}

/// A vertical rule helper.
pub struct Vr {
    pub id: String,
    pub style: Style,
}

impl Vr {
    pub fn new() -> Self {
        let mut style = Style::default();
        style.sizing.width = Some(1.0);
        style.sizing.min_height = Some(16.0);
        style.background.color = Some("currentColor".into());
        style.layout.display = Display::InlineBlock;
        style.layout.visibility = crate::utils::Visibility::Visible;
        Self { id: generate_id(), style }
    }
}

impl Component for Vr {
    fn id(&self) -> &str { &self.id }
    fn render(&self) { log::debug!("Rendering VR [{}]", self.id); }
}

/// An aspect ratio container helper.
pub struct Ratio {
    pub id: String,
    pub ratio: f32,
    pub style: Style,
    pub children: Children,
}

impl Ratio {
    pub fn new(ratio: f32) -> Self {
        let mut style = Style::default();
        style.layout.aspect_ratio = Some(ratio);
        Self { id: generate_id(), ratio, style, children: Children::new() }
    }

    pub fn child(mut self, child: Box<dyn Component>) -> Self { self.children.add(child); self }
}

impl Component for Ratio {
    fn id(&self) -> &str { &self.id }
    fn render(&self) { self.children.render_all(); }
}
