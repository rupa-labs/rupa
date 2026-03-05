use crate::utils::color::Color;
use crate::utils::background::{BackgroundRepeat, BackgroundSize, BackgroundClip, BackgroundOrigin};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum MaskType {
    #[default]
    Luminance,
    Alpha,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum MaskMode {
    #[default]
    MatchSource,
    Luminance,
    Alpha,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum MaskComposite {
    #[default]
    Add,
    Subtract,
    Intersect,
    Exclude,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Shadow {
    pub offset: [f32; 2],
    pub blur: f32,
    pub spread: f32,
    pub color: Option<Color>,
}

impl Shadow {
    pub fn new(x: f32, y: f32, blur: f32, color: impl Into<Color>) -> Self {
        Self {
            offset: [x, y],
            blur,
            spread: 0.0,
            color: Some(color.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Mask {
    pub image: Option<String>,
    pub clip: BackgroundClip,
    pub composite: MaskComposite,
    pub mode: MaskMode,
    pub origin: BackgroundOrigin,
    pub position: [f32; 2],
    pub repeat: BackgroundRepeat,
    pub size: BackgroundSize,
    pub mask_type: MaskType,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Effects {
    pub opacity: Option<f32>,
    pub box_shadow: Vec<Shadow>,
    pub text_shadow: Vec<Shadow>,
    pub mix_blend_mode: BlendMode,
    pub background_blend_mode: Vec<BlendMode>,
    pub mask: Mask,
    pub z_index: Option<i32>,
}
