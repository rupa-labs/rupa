pub mod spring;
pub mod transition;
pub mod timeline;

pub use spring::Spring;
pub use transition::Transition;

/// An attribute that can be attached to VNodes to declare animations.
pub struct Motion;
