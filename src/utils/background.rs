use crate::utils::color::Color;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BackgroundAttachment {
    #[default]
    Scroll,
    Fixed,
    Local,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BackgroundClip {
    #[default]
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BackgroundOrigin {
    #[default]
    PaddingBox,
    BorderBox,
    ContentBox,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BackgroundRepeat {
    #[default]
    Repeat,
    NoRepeat,
    RepeatX,
    RepeatY,
    Round,
    Space,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum BackgroundSize {
    #[default]
    Auto,
    Cover,
    Contain,
    Length(f32, f32),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Background {
    pub color: Option<Color>,
    pub image: Option<String>,
    pub attachment: BackgroundAttachment,
    pub clip: BackgroundClip,
    pub origin: BackgroundOrigin,
    pub position: [f32; 2], // [x, y] percentage or logical pixels
    pub repeat: BackgroundRepeat,
    pub size: BackgroundSize,
}
