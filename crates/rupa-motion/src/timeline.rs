use std::sync::{Arc, RwLock};
use std::time::Instant;

type AnimationCallback = Box<dyn FnMut(f32) -> bool + Send + Sync>;

/// A global timeline to orchestrate active animations.
pub struct Timeline {
    animations: Arc<RwLock<Vec<AnimationCallback>>>,
    last_tick: Arc<RwLock<Option<Instant>>>,
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            animations: Arc::new(RwLock::new(Vec::new())),
            last_tick: Arc::new(RwLock::new(None)),
        }
    }

    /// Adds a new animation to the timeline.
    /// The closure should return `true` if the animation is still running, 
    /// or `false` if it has finished and can be removed.
    pub fn add(&self, animation: impl FnMut(f32) -> bool + Send + Sync + 'static) {
        let mut anims = self.animations.write().unwrap();
        anims.push(Box::new(animation));
    }

    /// Progresses the timeline by the elapsed time.
    /// Returns true if there are active animations.
    pub fn tick(&self) -> bool {
        let mut anims = self.animations.write().unwrap();
        if anims.is_empty() {
            *self.last_tick.write().unwrap() = None;
            return false;
        }

        let now = Instant::now();
        let mut last_tick = self.last_tick.write().unwrap();
        
        let dt = if let Some(last) = *last_tick {
            now.duration_since(last).as_secs_f32()
        } else {
            1.0 / 60.0 // Assume 60fps for the first tick
        };

        *last_tick = Some(now);

        // Keep animations that return true (still running)
        anims.retain_mut(|anim| anim(dt));

        !anims.is_empty()
    }

    /// Checks if there are any active animations.
    pub fn is_active(&self) -> bool {
        !self.animations.read().unwrap().is_empty()
    }
}

// Global Timeline Instance
lazy_static::lazy_static! {
    pub static ref GLOBAL_TIMELINE: Timeline = Timeline::new();
}
