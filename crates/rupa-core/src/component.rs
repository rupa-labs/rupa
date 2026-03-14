use rupa_vnode::{VNode, Style};
use std::any::Any;

/// # Rupa Component 🎨
/// 
/// The Artisan's Blueprint for UI elements. 
/// A component integrates five core pillars: Logic, State, Attributes, Behavior, and Styling.
pub trait Component: Send + Sync {
    /// --- Semantic Identity ---

    /// Returns a unique debug name for this component.
    fn debug_name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// --- Pillar 1: Logic (Lifecycle) ---

    /// Called when the component is mounted. Use this to trigger [listen] 
    /// or perform one-time initializations.
    fn mount(&self) {
        self.listen();
    }

    /// Called before the component is destroyed.
    fn unmount(&self) {}

    /// --- Pillar 2: Styling (Visual DNA) ---

    /// Defines the base aesthetic rules for this component.
    /// Standardizing this allows for theme injection and consistent scaling.
    fn style(&self) -> Style {
        Style::default()
    }

    /// --- Pillar 3: Behavior (Events & Reactivity) ---

    /// Used to register global listeners, signal effects, or internal bus subscriptions.
    /// By default, this is called during [mount].
    fn listen(&self) {}

    /// --- Pillar 4 & 5: Visual Structure & State ---

    /// Called immediately before [render].
    fn rendering(&self) {}

    /// The mandatory visual contract. Produces the [VNode] tree.
    /// State and Attributes (fields) are consumed here to drive the UI.
    fn render(&self) -> VNode;

    /// Called after the changes have been patched to the platform.
    fn rendered(&self) {}

    // --- State Hooks (Inter-signal Coordination) ---

    fn updating(&self) {}
    fn updated(&self) {}

    // --- Capabilities ---

    /// If true, this component acts as a modal / focus trap.
    fn is_modal(&self) -> bool {
        false
    }

    /// Allows downcasting this trait object to a concrete type.
    fn as_any(&self) -> &dyn Any;

    // --- Infrastructure (Internal) ---

    fn prev_vnode(&self) -> std::sync::Arc<std::sync::RwLock<Option<VNode>>>;
    
    fn children(&self) -> Vec<&dyn Component> {
        Vec::new()
    }

    fn is_dirty(&self) -> bool {
        true
    }

    fn clear_dirty(&self) {}
}

/// A thread-safe, heap-allocated component handle.
pub type ComponentPtr = std::sync::Arc<dyn Component>;
