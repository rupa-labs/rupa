use rupa_core::{Component, VNode, VElement, Vec2, ViewCore, generate_id, Signal, Readable, Renderer, TextMeasurer, SceneNode, UIEvent, EventListeners, CursorIcon};
use rupa_vnode::{Style, Color, Theme, Variant, Spacing, Scale, Accessibility, TextAlign, SemanticRole, Attributes};
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
