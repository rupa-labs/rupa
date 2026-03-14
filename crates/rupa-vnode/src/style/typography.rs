use crate::color::Color;
use crate::types::{TextAlign, FontWeight};
use serde::{Serialize, Deserialize};

/// Visual properties for text elements.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TypographyStyle {
    pub family: FontFamily,
    pub size: f32,
    pub weight: FontWeight,
    pub align: TextAlign,
    pub italic: bool,
    pub decoration: TextDecoration,
    pub transform: TextTransform,
    pub wrap: TextWrap,
    pub overflow: TextOverflow,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub color: Option<Color>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum FontFamily {
    #[default]
    Sans,
    Serif,
    Mono,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TextDecoration {
    #[default]
    None,
    Underline,
    LineThrough,
    Overline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TextTransform {
    #[default]
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TextWrap {
    #[default]
    Wrap,
    NoWrap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
}
