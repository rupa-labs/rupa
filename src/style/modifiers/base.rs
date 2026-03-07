use crate::style::utilities::style::Style;
use crate::core::component::Component;
use std::sync::RwLockWriteGuard;

/// A trait for modifying styles in a functional or chaining manner.
pub trait StyleModifier {
    fn apply(&self, style: &mut Style);
}

impl<F> StyleModifier for F
where
    F: Fn(&mut Style),
{
    fn apply(&self, style: &mut Style) {
        (self)(style);
    }
}

/// A trait for components that can be styled.
/// This provides the glue between the component and the modifier system.
pub trait Stylable: Component + Sized {
    /// Returns a write lock guard to the component's internal style.
    fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style>;

    /// Applies a style modifier to the component.
    fn style(self, modifier: impl StyleModifier) -> Self {
        {
            let mut style = self.get_style_mut();
            modifier.apply(&mut *style);
        }
        self.mark_dirty();
        self
    }
}
