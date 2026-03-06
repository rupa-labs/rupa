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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_defaults() {
        let motion = Motion::default();
        assert!(motion.transitions.is_empty());
        assert!(motion.animations.is_empty());
    }

    #[test]
    fn test_transition_creation() {
        let t = Transition {
            property: "opacity".to_string(),
            duration: 300.0,
            timing: TimingFunction::EaseIn,
            ..Default::default()
        };
        assert_eq!(t.property, "opacity");
        assert_eq!(t.duration, 300.0);
        assert_eq!(t.timing, TimingFunction::EaseIn);
    }
}
