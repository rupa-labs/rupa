use rupa_core::events::{use_hover, use_active, use_focus};
use rupa_signals::Effect;
use crate::style::modifiers::base::{StyleModifier, Stylable};

/// A trait for components that support high-level behaviors.
/// It uses the internal id() and get_style() methods from the struct implementation.
pub trait Behavioral: Stylable + Sized {
    /// Reactive hover behavior.
    fn on_hover_style(self, modifier: impl StyleModifier + Send + Sync + 'static) -> Self {
        // Accessing methods from the concrete struct (via Stylable constraint)
        let id = self.id().to_string();
        let style = self.get_style();
        let is_hovered = use_hover(id);
        
        Effect::new(move || {
            if is_hovered.get() {
                let mut s = style.write().unwrap();
                modifier.apply(&mut s);
            }
        });
        self
    }

    /// Reactive active/pressed behavior.
    fn on_active_style(self, modifier: impl StyleModifier + Send + Sync + 'static) -> Self {
        let id = self.id().to_string();
        let style = self.get_style();
        let is_active = use_active(id);
        
        Effect::new(move || {
            if is_active.get() {
                let mut s = style.write().unwrap();
                modifier.apply(&mut s);
            }
        });
        self
    }

    /// Reactive focus behavior.
    fn on_focus_style(self, modifier: impl StyleModifier + Send + Sync + 'static) -> Self {
        let id = self.id().to_string();
        let style = self.get_style();
        let is_focused = use_focus(id);
        
        Effect::new(move || {
            if is_focused.get() {
                let mut s = style.write().unwrap();
                modifier.apply(&mut s);
            }
        });
        self
    }
}

impl<T: Stylable> Behavioral for T {}
