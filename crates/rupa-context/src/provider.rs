use rupa_core::Component;

/// A high-level component that injects a value into the component tree.
pub struct Provider<T: Send + Sync + 'static> {
    pub value: T,
    pub child: Box<dyn Component>,
}

impl<T: Send + Sync + 'static> Provider<T> {
    pub fn new(value: T, child: impl Component + 'static) -> Self {
        Self {
            value,
            child: Box::new(child),
        }
    }
}

// Logic for VNode Tree propagation would go here
// In Rupa, we'll store the context in the ViewCore during the Build phase.
