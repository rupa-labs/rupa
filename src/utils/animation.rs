#[derive(Clone, Debug, PartialEq, Default)]
pub enum TimingFunction {
    #[default] Ease, Linear, EaseIn, EaseOut, EaseInOut, StepStart, StepEnd,
    CubicBezier(f32, f32, f32, f32),
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TransitionBehavior { #[default] Normal, AllowDiscrete }

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Transition {
    pub property: String,
    pub duration: f32, // milliseconds
    pub timing: TimingFunction,
    pub delay: f32,
    pub behavior: TransitionBehavior,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Animation {
    pub name: String,
    pub duration: f32,
    pub timing: TimingFunction,
    pub delay: f32,
    pub iteration_count: String, // "infinite" or "1", "2"
    pub direction: String, // "normal", "reverse", "alternate"
    pub fill_mode: String, // "none", "forwards", "backwards", "both"
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Motion {
    pub transitions: Vec<Transition>,
    pub animations: Vec<Animation>,
}
