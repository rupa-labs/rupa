use std::time::Duration;
use rupa_signals::Signal;
use crate::{Animatable, easing::Easing};
use crate::timeline::GLOBAL_TIMELINE;

/// A duration-based animation configuration.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransitionConfig {
    pub duration: Duration,
    pub easing: Easing,
    pub delay: Duration,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(300),
            easing: Easing::CubicInOut,
            delay: Duration::ZERO,
        }
    }
}

/// A handle to a duration-based animation.
pub struct Transition<T: Animatable> {
    pub value: Signal<T>,
    pub config: TransitionConfig,
}

impl<T: Animatable> Transition<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Signal::new(initial),
            config: TransitionConfig::default(),
        }
    }

    pub fn with_config(initial: T, config: TransitionConfig) -> Self {
        Self {
            value: Signal::new(initial),
            config,
        }
    }

    /// Transitions the value toward a new target.
    pub fn set_target(&self, target: T) {
        let value_signal = self.value.clone();
        let config = self.config;
        let start_val = value_signal.get();
        let mut elapsed = 0.0;
        let duration_secs = config.duration.as_secs_f32();

        GLOBAL_TIMELINE.add(move |dt| {
            elapsed += dt;
            
            if elapsed >= duration_secs {
                value_signal.set(target.clone());
                return false; // Finished
            }

            let t = elapsed / duration_secs;
            let eased_t = config.easing.apply(t);
            let interpolated = start_val.interpolate(&target, eased_t);
            value_signal.set(interpolated);
            
            true // Continue
        });
    }

    pub fn get(&self) -> T {
        self.value.get()
    }
}
