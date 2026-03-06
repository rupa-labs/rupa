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
}

pub trait IntoSpacing {
    fn into_spacing(self) -> Spacing;
}

impl IntoSpacing for f32 {
    fn into_spacing(self) -> Spacing {
        Spacing::all(self)
    }
}
