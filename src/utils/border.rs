#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rounding {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
    pub bottom_left: f32,
}

impl Rounding {
    pub fn all(val: f32) -> Self {
        Self {
            top_left: val,
            top_right: val,
            bottom_right: val,
            bottom_left: val,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Border {
    pub width: f32,
    pub color: [f32; 4], // RGBA
}
