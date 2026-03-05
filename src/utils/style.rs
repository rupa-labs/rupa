use super::spacing::Spacing;
use super::border::{Border, Rounding};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Style {
    pub margin: Spacing,
    pub padding: Spacing,
    pub border: Border,
    pub rounding: Rounding,
    pub background_color: Option<[f32; 4]>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    // Utility-First API (Tailwind-like)
    
    pub fn m(mut self, val: f32) -> Self {
        self.margin = Spacing::all(val);
        self
    }

    pub fn p(mut self, val: f32) -> Self {
        self.padding = Spacing::all(val);
        self
    }

    pub fn px(mut self, val: f32) -> Self {
        self.padding = self.padding.x(val);
        self
    }

    pub fn py(mut self, val: f32) -> Self {
        self.padding = self.padding.y(val);
        self
    }

    pub fn rounded(mut self, val: f32) -> Self {
        self.rounding = Rounding::all(val);
        self
    }

    pub fn bg(mut self, r: f32, g: f32, b: f32, a: f32) -> Self {
        self.background_color = Some([r, g, b, a]);
        self
    }
}
