use rupa_core::vnode::VNode; use rupa_core::component::Component;
use rupa_styling::Style;
use rupa_core::component::Component;
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

impl StyleModifier for Style {
    fn apply(&self, style: &mut Style) {
        *style = self.clone();
    }
}

// --- Tuple Implementations for Multi-Modifiers ---

impl<A, B> StyleModifier for (A, B) 
where A: StyleModifier, B: StyleModifier {
    fn apply(&self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
    }
}

impl<A, B, C> StyleModifier for (A, B, C) 
where A: StyleModifier, B: StyleModifier, C: StyleModifier {
    fn apply(&self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
    }
}

impl<A, B, C, D> StyleModifier for (A, B, C, D) 
where A: StyleModifier, B: StyleModifier, C: StyleModifier, D: StyleModifier {
    fn apply(&self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
        self.3.apply(style);
    }
}

impl<A, B, C, D, E> StyleModifier for (A, B, C, D, E) 
where A: StyleModifier, B: StyleModifier, C: StyleModifier, D: StyleModifier, E: StyleModifier {
    fn apply(&self, style: &mut Style) {
        self.0.apply(style);
        self.1.apply(style);
        self.2.apply(style);
        self.3.apply(style);
        self.4.apply(style);
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
