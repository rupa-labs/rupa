use std::time::Duration;
use rupa_vnode::{Style, motion_mod::{Transition, Easing}};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn transition(duration: Duration, easing: Easing) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut m = s.motion.clone().unwrap_or_default();
        m.transition = Some(Transition {
            duration: duration.as_millis() as f32,
            delay: 0.0,
            easing: easing.clone(),
        });
        s.motion = Some(m);
    }
}

pub fn spring() -> impl StyleModifier {
    move |s: &mut Style| {
        let mut m = s.motion.clone().unwrap_or_default();
        m.spring = Some(Default::default());
        s.motion = Some(m);
    }
}

// --- Chaining API ---

pub trait ChainedMotion: Stylable {
    fn transition(self, duration: Duration, easing: Easing) -> Self { 
        self.style(transition(duration, easing)) 
    }

    fn spring(self) -> Self {
        self.style(spring())
    }
}

impl<T: Stylable> ChainedMotion for T {}
