#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spacing {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Spacing {
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

pub trait IntoSpacing {
    fn into_spacing(self) -> Spacing;
}

impl IntoSpacing for f32 {
    fn into_spacing(self) -> Spacing {
        Spacing::all(self)
    }
}

impl IntoSpacing for (f32, f32) {
    fn into_spacing(self) -> Spacing {
        Spacing { top: self.0, right: self.1, bottom: self.0, left: self.1 }
    }
}

impl IntoSpacing for (f32, f32, f32, f32) {
    fn into_spacing(self) -> Spacing {
        Spacing { top: self.0, right: self.1, bottom: self.2, left: self.3 }
    }
}
