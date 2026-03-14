use crate::color::Color;
use crate::types::{TextAlign, FontWeight, Unit};
use serde::{Serialize, Deserialize};

/// Visual properties for text elements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypographyStyle {
    pub family: FontFamily,
    pub size: Unit,
    pub weight: FontWeight,
    pub align: TextAlign,
    pub italic: bool,
    pub decoration: TextDecoration,
    pub transform: TextTransform,
    pub wrap: TextWrap,
    pub overflow: TextOverflow,
    pub line_height: Unit,
    pub letter_spacing: Unit,
    pub color: Option<Color>,
}

impl Default for TypographyStyle {
    fn default() -> Self {
        Self {
            family: FontFamily::default(),
            size: Unit::Absolute(16.0),
            weight: FontWeight::default(),
            align: TextAlign::default(),
            italic: false,
            decoration: TextDecoration::default(),
            transform: TextTransform::default(),
            wrap: TextWrap::default(),
            overflow: TextOverflow::default(),
            line_height: Unit::Absolute(1.2),
            letter_spacing: Unit::Absolute(0.0),
            color: None,
        }
    }
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
