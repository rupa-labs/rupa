use std::time::Duration;
use rupa_vnode::{Style, Transition, Easing};
use super::base::{StyleModifier, Stylable};

// --- Functional API ---

pub fn motion(duration: Duration, easing: Easing) -> impl StyleModifier {
    move |s: &mut Style| {
        let mut m = s.motion.clone().unwrap_or_default();
        m.transitions.push(Transition {
            property: "all".into(), // Default to all for now
            duration: duration.as_millis() as f32,
            timing: easing.clone(),
            ..Default::default()
        });
        s.motion = Some(m);
    }
}

// --- Chaining API ---

pub trait ChainedMotion: Stylable {
    fn motion(self, duration: Duration, easing: Easing) -> Self { 
        self.style(motion(duration, easing)) 
    }
}

impl<T: Stylable> ChainedMotion for T {}
