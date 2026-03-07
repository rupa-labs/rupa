use crate::color::Color;
use crate::types::{TextAlign, FontWeight};
use serde::{Serialize, Deserialize};

/// Visual properties for text elements.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TypographyStyle {
    pub size: f32,
    pub weight: FontWeight,
    pub align: TextAlign,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub color: Option<Color>,
}
