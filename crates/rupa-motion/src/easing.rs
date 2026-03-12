use std::f32::consts::PI;

/// A collection of standard Easing functions for smooth transitions.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize, Default)]
pub enum Easing {
    #[default]
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    BackIn,
    BackOut,
    BackInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
}

impl Easing {
    pub fn apply(&self, t: f32) -> f32 {
        match self {
            Easing::Linear => t,
            Easing::QuadIn => t * t,
            Easing::QuadOut => t * (2.0 - t),
            Easing::QuadInOut => {
                if t < 0.5 { 2.0 * t * t } 
                else { -1.0 + (4.0 - 2.0 * t) * t }
            },
            Easing::CubicIn => t * t * t,
            Easing::CubicOut => { let t = t - 1.0; t * t * t + 1.0 },
            Easing::CubicInOut => {
                if t < 0.5 { 4.0 * t * t * t }
                else { (t - 1.0) * (2.0 * t - 2.0) * (2.0 * t - 2.0) + 1.0 }
            },
            Easing::QuartIn => t * t * t * t,
            Easing::QuartOut => { let t = t - 1.0; 1.0 - t * t * t * t },
            Easing::QuartInOut => {
                if t < 0.5 { 8.0 * t * t * t * t }
                else { let t = t - 1.0; 1.0 - 8.0 * t * t * t * t }
            },
            Easing::ExpoIn => if t == 0.0 { 0.0 } else { (2.0f32).powf(10.0 * (t - 1.0)) },
            Easing::ExpoOut => if t == 1.0 { 1.0 } else { 1.0 - (2.0f32).powf(-10.0 * t) },
            Easing::ExpoInOut => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else if t < 0.5 { (2.0f32).powf(10.0 * (2.0 * t - 1.0)) / 2.0 }
                else { (2.0 - (2.0f32).powf(-10.0 * (2.0 * t - 1.0))) / 2.0 }
            },
            Easing::BackIn => {
                let s = 1.70158;
                t * t * ((s + 1.0) * t - s)
            },
            Easing::BackOut => {
                let s = 1.70158;
                let t = t - 1.0;
                t * t * ((s + 1.0) * t + s) + 1.0
            },
            Easing::BackInOut => {
                let s = 1.70158 * 1.525;
                let t = t * 2.0;
                if t < 1.0 { 0.5 * (t * t * ((s + 1.0) * t - s)) }
                else { let t = t - 2.0; 0.5 * (t * t * ((s + 1.0) * t + s) + 2.0) }
            },
            Easing::ElasticIn => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else {
                    let p = 0.3;
                    let s = p / 4.0;
                    let t = t - 1.0;
                    -((2.0f32).powf(10.0 * t) * ((t - s) * (2.0 * PI) / p).sin())
                }
            },
            Easing::ElasticOut => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else {
                    let p = 0.3;
                    let s = p / 4.0;
                    (2.0f32).powf(-10.0 * t) * ((t - s) * (2.0 * PI) / p).sin() + 1.0
                }
            },
            Easing::ElasticInOut => {
                let p = 0.3 * 1.5;
                let s = p / 4.0;
                let t = t * 2.0;
                if t == 0.0 { 0.0 }
                else if t == 2.0 { 1.0 }
                else if t < 1.0 {
                    let t = t - 1.0;
                    -0.5 * ((2.0f32).powf(10.0 * t) * ((t - s) * (2.0 * PI) / p).sin())
                }
                else {
                    let t = t - 1.0;
                    0.5 * ((2.0f32).powf(-10.0 * t) * ((t - s) * (2.0 * PI) / p).sin()) + 1.0
                }
            },
        }
    }
}
