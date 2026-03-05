use crate::utils::{Style, StyleModifier, PathData, generate_id};
use crate::Component;
use crate::container::Children;

/// The root semantic component for vector graphics.
pub struct SvgCanvas {
    pub id: String,
    pub style: Style,
    pub children: Children,
}

impl SvgCanvas {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
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

impl Component for SvgCanvas {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering SVG Canvas [ID: {}]", self.id);
        self.children.render_all();
    }
}

/// A semantic component representing a vector path.
pub struct Path {
    pub id: String,
    pub data: PathData,
    pub style: Style,
}

impl Path {
    pub fn new() -> Self {
        Self {
            id: generate_id(),
            data: PathData::new(),
            style: Style::default(),
        }
    }

    /// Set the vector path data.
    pub fn data(mut self, data: PathData) -> Self {
        self.data = data;
        self
    }

    pub fn style(mut self, modifier: impl StyleModifier) -> Self {
        modifier.apply(&mut self.style);
        self
    }
}

impl Component for Path {
    fn id(&self) -> &str { &self.id }
    fn render(&self) {
        log::debug!("Rendering Path [ID: {}]: {}", self.id, self.data.to_svg_string());
    }
}
