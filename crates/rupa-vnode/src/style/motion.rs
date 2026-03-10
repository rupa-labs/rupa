use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum TimingFunction {
    #[default] Ease, Linear, EaseIn, EaseOut, EaseInOut, StepStart, StepEnd,
    CubicBezier(f32, f32, f32, f32),
    Spring, 
}

pub type Easing = TimingFunction;

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum TransitionBehavior { #[default] Normal, AllowDiscrete }

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transition {
    pub property: String,
    pub duration: f32, // milliseconds
    pub timing: TimingFunction,
    pub delay: f32,
    pub behavior: TransitionBehavior,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    pub name: String,
    pub duration: f32,
    pub timing: TimingFunction,
    pub delay: f32,
    pub iteration_count: String, 
    pub direction: String, 
    pub fill_mode: String, 
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Motion {
    pub transitions: Vec<Transition>,
    pub animations: Vec<Animation>,
}
