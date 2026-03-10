use rupa_signals::Signal;
use rupa_vnode::style::motion::SpringConfig;
use crate::Animatable;
use std::sync::{Arc, RwLock};
use crate::timeline::GLOBAL_TIMELINE;

/// A physics-based interpolator that smoothly moves a value toward a target.
pub struct Spring<T: Animatable> {
    pub value: Signal<T>,
    pub target: Arc<RwLock<T>>,
    pub velocity: Arc<RwLock<f32>>,
    pub config: SpringConfig,
}

impl<T: Animatable> Spring<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Signal::new(initial.clone()),
            target: Arc::new(RwLock::new(initial)),
            velocity: Arc::new(RwLock::new(0.0)),
            config: SpringConfig::default(),
        }
    }

    pub fn with_config(initial: T, config: SpringConfig) -> Self {
        Self {
            value: Signal::new(initial.clone()),
            target: Arc::new(RwLock::new(initial)),
            velocity: Arc::new(RwLock::new(0.0)),
            config,
        }
    }

    pub fn set_target(&self, target: T) {
        *self.target.write().unwrap() = target.clone();
        
        let value_signal = self.value.clone();
        let target_arc = self.target.clone();
        let velocity_arc = self.velocity.clone();
        let config = self.config.clone();

        // This is a simplified 1D spring simulation used for interpolation
        // For actual T types (like Vec2), we interpolate based on a 1D spring progress
        // from 0.0 to 1.0, where target is 1.0.
        
        // Let's create a 1D spring that interpolates from 0 to 1
        let mut current_progress = 0.0;
        let mut vel = *velocity_arc.read().unwrap();
        let start_val = value_signal.get();

        GLOBAL_TIMELINE.add(move |dt| {
            // Spring force: F = -k * x
            let displacement = 1.0 - current_progress;
            let spring_force = config.stiffness * displacement;

            // Damping force: F = -c * v
            let damping_force = config.damping * vel;

            // Acceleration: a = F / m
            let acceleration = (spring_force - damping_force) / config.mass;

            vel += acceleration * dt;
            current_progress += vel * dt;

            // Update global velocity tracker
            *velocity_arc.write().unwrap() = vel;

            let target_val = target_arc.read().unwrap().clone();

            if (1.0 - current_progress).abs() < config.precision && vel.abs() < config.precision {
                value_signal.set(target_val);
                *velocity_arc.write().unwrap() = 0.0;
                false // Animation finished
            } else {
                let interpolated = start_val.interpolate(&target_val, current_progress);
                value_signal.set(interpolated);
                true // Continue animation
            }
        });
    }

    pub fn get(&self) -> T {
        self.value.get()
    }
}
