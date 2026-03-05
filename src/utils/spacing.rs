#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self { top, right, bottom, left }
    }

    pub fn all(val: f32) -> Self {
        Self { top: val, right: val, bottom: val, left: val }
    }

    pub fn x(mut self, val: f32) -> Self {
        self.left = val;
        self.right = val;
        self
    }

    pub fn y(mut self, val: f32) -> Self {
        self.top = val;
        self.bottom = val;
        self
    }
}
