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

/// # Rupa Step 📐
/// 
/// A semantic, multi-step scaling system for spacing and sizing.
/// Ensures aesthetic consistency across the entire application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Step {
    Zero,
    S1, S2, S3, S4, S5, S6, S7, S8, S9, S10,
    S12, S16, S20, S24, S32, S40, S48, S56, S64,
}

impl Default for Step {
    fn default() -> Self {
        Step::Zero
    }
}

/// # Rupa Unit 📏
/// 
/// A flexible unit that can be either a semantic [Step] or an [Absolute] decimal value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Unit {
    /// A semantic step in the unified scale.
    Step(Step),
    /// An absolute decimal value (Pixels in GUI, Cells in TUI).
    Absolute(f32),
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Absolute(0.0)
    }
}

impl From<f32> for Unit {
    fn from(v: f32) -> Self {
        Unit::Absolute(v)
    }
}

impl From<Step> for Unit {
    fn from(s: Step) -> Self {
        Unit::Step(s)
    }
}

impl Unit {
    /// Resolves the unit into a physical f32 value.
    /// [Step] values are multiplied by the provided base (usually 4.0 or 8.0).
    pub fn resolve(&self, base: f32) -> f32 {
        match self {
            Unit::Absolute(v) => *v,
            Unit::Step(s) => match s {
                Step::Zero => 0.0,
                Step::S1 => base * 1.0,
                Step::S2 => base * 2.0,
                Step::S3 => base * 3.0,
                Step::S4 => base * 4.0,
                Step::S5 => base * 5.0,
                Step::S6 => base * 6.0,
                Step::S7 => base * 7.0,
                Step::S8 => base * 8.0,
                Step::S9 => base * 9.0,
                Step::S10 => base * 10.0,
                Step::S12 => base * 12.0,
                Step::S16 => base * 16.0,
                Step::S20 => base * 20.0,
                Step::S24 => base * 24.0,
                Step::S32 => base * 32.0,
                Step::S40 => base * 40.0,
                Step::S48 => base * 48.0,
                Step::S56 => base * 56.0,
                Step::S64 => base * 64.0,
            }
        }
    }
}

impl std::ops::Mul<f32> for Unit {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        self.resolve(4.0) * rhs
    }
}

// Deprecating old Scale in favor of Step/Unit
#[deprecated(note = "Use Step and Unit instead")]
pub type Scale = Step;
