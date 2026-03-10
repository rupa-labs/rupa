use rupa_core::Component;
pub mod container;
pub mod hstack;
pub mod section;
pub mod vstack;

pub use container::Container;
pub use hstack::HStack;
pub use section::Section;
pub use vstack::VStack;


pub trait Parent: Component + Sized {
    fn child(self, child: Box<dyn Component>) -> Self;
}
