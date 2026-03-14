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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
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
    Custom(u32),
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
            Breakpoint::Custom(v) => *v as f32,
        }
    }
}

/// # Rupa Scale 📐
/// 
/// The official 10-step semantic scaling system of the Rupa Framework.
/// Ensures aesthetic consistency through curated size classes.
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
}

/// # Rupa Unit 📏
/// 
/// A flexible unit that can be either a semantic [Scale] or an [Absolute] decimal value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    /// A semantic size class in the 10-step scale.
    Scale(Scale),
    /// An absolute decimal value (Pixels in GUI, Cells in TUI).
    Absolute(f32),
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Scale(Scale::None)
    }
}

impl From<f32> for Unit {
    fn from(v: f32) -> Self {
        Unit::Absolute(v)
    }
}

impl From<Scale> for Unit {
    fn from(s: Scale) -> Self {
        Unit::Scale(s)
    }
}

impl Unit {
    /// Resolves the unit into a physical f32 value.
    /// [Scale] variants map to an opinionated geometric progression.
    pub fn resolve(&self, base: f32) -> f32 {
        match self {
            Unit::Absolute(v) => *v,
            Unit::Scale(s) => match s {
                Scale::None => 0.0,
                Scale::Xs => base * 0.25,  // 4
                Scale::Sm => base * 0.5,   // 8
                Scale::Md => base,         // 16
                Scale::Lg => base * 1.5,   // 24
                Scale::Xl => base * 2.0,   // 32
                Scale::Xl2 => base * 3.0,  // 48
                Scale::Xl3 => base * 4.0,  // 64
                Scale::Xl4 => base * 6.0,  // 96
                Scale::Xl5 => base * 8.0,  // 128
                Scale::Xl6 => base * 12.0, // 192
            }
        }
    }
}

impl std::ops::Mul<f32> for Unit {
    type Output = f32;
    fn mul(self, rhs: f32) -> Self::Output {
        self.resolve(16.0) * rhs
    }
}
