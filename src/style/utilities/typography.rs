use crate::style::utilities::color::Color;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontWeight { Thin, ExtraLight, Light, #[default] Regular, Medium, SemiBold, Bold, ExtraBold, Black }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontStyle { #[default] Normal, Italic, Oblique }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextAlign { #[default] Left, Center, Right, Justify }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextDecoration { #[default] None, Underline, Overline, LineThrough }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Typography {
    pub family: Option<String>,
    pub size: Option<f32>,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub align: TextAlign,
    pub decoration: TextDecoration,
    pub color: Option<Color>,
    pub line_height: Option<f32>,
    pub letter_spacing: Option<f32>,
}

impl Typography {
    pub fn base_size(&self) -> f32 { self.size.unwrap_or(16.0) }
}
