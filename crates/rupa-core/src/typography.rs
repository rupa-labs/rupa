/// Standard text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

/// Font weight definitions following standard CSS-like values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum FontWeight {
    Thin = 100,
    Light = 300,
    #[default]
    Normal = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    Black = 900,
}

/// Visual properties for text elements.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TypographyStyle {
    pub size: f32,
    pub weight: FontWeight,
    pub align: TextAlign,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub color: Option<crate::style::utilities::color::Color>,
}
