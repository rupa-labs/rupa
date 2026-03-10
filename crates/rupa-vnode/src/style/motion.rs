use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Easing {
    #[default]
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    BackIn,
    BackOut,
    CubicBezier(f32, f32, f32, f32),
}

/// Configuration for a duration-based transition.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transition {
    pub duration: f32, // milliseconds
    pub delay: f32,    // milliseconds
    pub easing: Easing,
}

impl Transition {
    pub fn new(millis: f32) -> Self {
        Self { duration: millis, delay: 0.0, easing: Easing::EaseInOut }
    }
}

/// Configuration for spring physics.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpringConfig {
    pub stiffness: f32,
    pub damping: f32,
    pub mass: f32,
    pub precision: f32,
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self {
            stiffness: 170.0,
            damping: 26.0,
            mass: 1.0,
            precision: 0.01,
        }
    }
}

/// Declarative motion intent for a UI element.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Motion {
    pub transition: Option<Transition>,
    pub spring: Option<SpringConfig>,
}

impl Motion {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_transition(transition: Transition) -> Self {
        Self { transition: Some(transition), spring: None }
    }

    pub fn with_spring(config: SpringConfig) -> Self {
        Self { transition: None, spring: Some(config) }
    }
}
