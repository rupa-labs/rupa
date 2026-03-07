use serde::{Serialize, Deserialize};
use crate::color::Color;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub width: f32,
    pub color: Option<Color>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rounding {
    pub nw: f32,
    pub ne: f32,
    pub se: f32,
    pub sw: f32,
}

impl Rounding {
    pub fn all(val: f32) -> Self {
        Self { nw: val, ne: val, se: val, sw: val }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Outline {
    pub width: f32,
    pub offset: f32,
    pub color: Option<Color>,
}
