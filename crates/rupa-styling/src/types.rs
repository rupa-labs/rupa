use serde::{Serialize, Deserialize};

/// Standard text alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
    Justify,
}

/// Font weight definitions following standard CSS-like values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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

/// Responsive design breakpoints based on standard screen widths.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Default, Serialize, Deserialize)]
pub enum Breakpoint {
    #[default]
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
}

impl Breakpoint {
    pub fn min_width(&self) -> f32 {
        match self {
            Breakpoint::Xs => 0.0,
            Breakpoint::Sm => 640.0,
            Breakpoint::Md => 768.0,
            Breakpoint::Lg => 1024.0,
            Breakpoint::Xl => 1280.0,
            Breakpoint::Xl2 => 1536.0,
            Breakpoint::Xl3 => 1920.0,
            Breakpoint::Xl4 => 2560.0,
            Breakpoint::Xl5 => 3840.0,
            Breakpoint::Xl6 => 5120.0,
        }
    }
}

/// A unified scale system for spacing, sizing, and typography.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Scale {
    #[default]
    None,
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
    Xl4,
    Xl5,
    Xl6,
    Custom(u32),
}

impl Scale {
    pub fn value(&self, base: f32) -> f32 {
        match self {
            Scale::None => 0.0,
            Scale::Xs => base * 0.25,
            Scale::Sm => base * 0.5,
            Scale::Md => base,
            Scale::Lg => base * 1.5,
            Scale::Xl => base * 2.0,
            Scale::Xl2 => base * 3.0,
            Scale::Xl3 => base * 4.0,
            Scale::Xl4 => base * 6.0,
            Scale::Xl5 => base * 8.0,
            Scale::Xl6 => base * 12.0,
            Scale::Custom(v) => *v as f32,
        }
    }
}
