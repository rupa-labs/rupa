use crate::color::Color;

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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_rounding_all() {
        let r = crate::support::Rounding::all(8.0);
        assert_eq!(r.nw, 8.0);
        assert_eq!(r.se, 8.0);
    }

    #[test]
    fn test_border_defaults() {
        let b = Border::default();
        assert_eq!(b.width, 0.0);
        assert_eq!(b.style, BorderStyle::None);
    }
}
