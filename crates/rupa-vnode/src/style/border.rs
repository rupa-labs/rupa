use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::types::Unit;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Border {
    pub width: Unit,
    pub color: Option<Color>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Rounding {
    pub nw: Unit,
    pub ne: Unit,
    pub se: Unit,
    pub sw: Unit,
}

impl Rounding {
    pub fn all(val: impl Into<Unit>) -> Self {
        let u = val.into();
        Self { nw: u, ne: u, se: u, sw: u }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Outline {
    pub width: Unit,
    pub offset: Unit,
    pub color: Option<Color>,
}
