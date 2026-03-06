use std::cell::{Cell, RefCell, RefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::support::Style;
use crate::scene::SceneNode;

/// The standardized infrastructure for any Component View.
/// Handles styling, geometric nodes, and invalidation.
/// Follows "Composition over Inheritance".
pub struct ViewCore {
    pub style: RefCell<Style>,
    pub node: Cell<Option<SceneNode>>,
    pub dirty: AtomicBool,
}

impl ViewCore {
    pub fn new(style: Style) -> Self {
        Self {
            style: RefCell::new(style),
            node: Cell::new(None),
            dirty: AtomicBool::new(true),
        }
    }

    pub fn default() -> Self {
        Self::new(Style::default())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    pub fn mark_dirty(&self) {
        self.dirty.store(true, Ordering::Relaxed);
    }

    pub fn clear_dirty(&self) {
        self.dirty.store(false, Ordering::Relaxed);
    }

    pub fn get_style_mut(&self) -> RefMut<'_, Style> {
        self.style.borrow_mut()
    }

    pub fn set_node(&self, node: SceneNode) {
        self.node.set(Some(node));
    }

    pub fn get_node(&self) -> Option<SceneNode> {
        self.node.get()
    }
}
