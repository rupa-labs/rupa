use rupa_signals::Signal;

/// A reactive signal that interpolates its value based on spring physics.
pub struct Spring<T> {
    pub value: Signal<T>,
    pub target: Signal<T>,
    pub stiffness: f32,
    pub damping: f32,
}

impl Spring<f32> {
    pub fn new(initial: f32) -> Self {
        Self {
            value: Signal::new(initial),
            target: Signal::new(initial),
            stiffness: 100.0,
            damping: 10.0,
        }
    }
}
