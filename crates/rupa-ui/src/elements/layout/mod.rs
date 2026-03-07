use rupa_core::vnode::VNode; use rupa_core::component::Component;
use rupa_core::component::Component;

/// A trait for components that can hold children.
/// Provides a unified API for adding children without manual redeclaration.
pub trait Parent: Component + Sized {
    /// Adds a child to the component.
    /// This requires the component to have a 'logic.children' field or similar.
    /// Because child storage is specific to each component's logic, 
    /// we can't provide a fully default implementation here without more complex traits,
    /// but we can unify the API signature.
    fn child(self, child: Box<dyn Component>) -> Self;
}

pub mod container;
pub mod hstack;
pub mod vstack;
pub mod section;

pub use container::Container;
pub use hstack::HStack;
pub use vstack::VStack;
pub use section::Section;

// Aliases for compatibility
pub type Row<'a> = HStack<'a>;
pub type Col<'a> = VStack<'a>;
