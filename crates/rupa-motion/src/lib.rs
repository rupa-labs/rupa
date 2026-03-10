pub mod spring;
pub mod timeline;

pub use spring::Spring;
pub use timeline::{Timeline, GLOBAL_TIMELINE};

/// Trait for types that can be animated/interpolated.
pub trait Animatable: Clone + Send + Sync + 'static {
    fn interpolate(&self, other: &Self, t: f32) -> Self;
}

impl Animatable for f32 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }
}

impl Animatable for rupa_base::Vec2 {
    fn interpolate(&self, other: &Self, t: f32) -> Self {
        rupa_base::Vec2::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t
        )
    }
}
