use crate::utils::color::Color;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontWeight {
    Thin, ExtraLight, Light, #[default] Regular, Medium, SemiBold, Bold, ExtraBold, Black,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontStyle { #[default] Normal, Italic, Oblique }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontSmoothing { #[default] Auto, Antialiased, SubpixelAntialiased }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontStretch {
    UltraCondensed, ExtraCondensed, Condensed, SemiCondensed,
    #[default] Normal,
    SemiExpanded, Expanded, ExtraExpanded, UltraExpanded,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextAlign { #[default] Left, Center, Right, Justify }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextTransform { #[default] None, Uppercase, Lowercase, Capitalize }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextDecorationLine { #[default] None, Underline, Overline, LineThrough }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextDecorationStyle { #[default] Solid, Double, Dotted, Dashed, Wavy }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextOverflow { #[default] Clip, Ellipsis }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextWrap { #[default] Wrap, NoWrap, Balance, Pretty }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WhiteSpace { #[default] Normal, NoWrap, Pre, PreLine, PreWrap }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WordBreak { #[default] Normal, BreakAll, KeepAll }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum OverflowWrap { #[default] Normal, Anywhere, BreakWord }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Hyphens { #[default] None, Manual, Auto }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum VerticalAlign { #[default] Baseline, Top, Middle, Bottom, Sub, Super }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ListStyleType { #[default] None, Disc, Circle, Square, Decimal, Roman }

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ListStylePosition { #[default] Outside, Inside }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Typography {
    pub family: Option<String>,
    pub size: Option<f32>,
    pub weight: FontWeight,
    pub style: FontStyle,
    pub smoothing: FontSmoothing,
    pub stretch: FontStretch,
    pub variant_numeric: Option<String>,
    pub feature_settings: Option<String>,
    pub letter_spacing: Option<f32>,
    pub line_height: Option<f32>,
    pub line_clamp: Option<u32>,
    
    pub color: Option<Color>,
    pub align: TextAlign,
    pub transform: TextTransform,
    pub decoration_line: TextDecorationLine,
    pub decoration_color: Option<Color>,
    pub decoration_style: TextDecorationStyle,
    pub decoration_thickness: Option<f32>,
    pub underline_offset: Option<f32>,
    
    pub overflow: TextOverflow,
    pub wrap: TextWrap,
    pub indent: Option<f32>,
    pub vertical_align: VerticalAlign,
    pub white_space: WhiteSpace,
    pub word_break: WordBreak,
    pub overflow_wrap: OverflowWrap,
    pub hyphens: Hyphens,
    
    pub list_type: ListStyleType,
    pub list_position: ListStylePosition,
    pub list_image: Option<String>,
    pub content: Option<String>,
}
