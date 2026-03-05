use crate::utils::color::Color;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BorderStyle {
    #[default]
    None,
    Hidden,
    Dotted,
    Dashed,
    Solid,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Border {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Option<Color>,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Outline {
    pub width: f32,
    pub style: BorderStyle,
    pub color: Option<Color>,
    pub offset: f32,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Rounding {
    pub nw: f32,
    pub ne: f32,
    pub sw: f32,
    pub se: f32,
}

impl Rounding {
    pub fn all(val: f32) -> Self {
        Self { nw: val, ne: val, sw: val, se: val }
    }
}
