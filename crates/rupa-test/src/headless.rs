use rupa_core::Component;
use rupa_vnode::VNode;

/// A virtual environment for running Rupa components without graphical output.
pub struct Tester;

impl Tester {
    pub fn new() -> Self {
        Self
    }

    /// Captures the virtual tree of a component.
    pub fn render(&self, component: &dyn Component) -> VNode {
        component.render()
    }
}
