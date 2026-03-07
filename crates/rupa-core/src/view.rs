use std::sync::{RwLock, RwLockWriteGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use crate::style_data::Style;
use crate::scene::SceneNode;

/// The standardized infrastructure for any Component View.
/// Handles styling, geometric nodes, and invalidation.
/// Follows "Composition over Inheritance" and is thread-safe.
pub struct ViewCore {
    pub style: RwLock<Style>,
    pub node: RwLock<Option<SceneNode>>,
    pub dirty: AtomicBool,
}

impl ViewCore {
    pub fn new(style: Style) -> Self {
        Self {
            style: RwLock::new(style),
            node: RwLock::new(None),
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

    pub fn get_style_mut(&self) -> RwLockWriteGuard<'_, Style> {
        self.style.write().unwrap()
    }

    pub fn set_node(&self, node: SceneNode) {
        *self.node.write().unwrap() = Some(node);
    }

    pub fn get_node(&self) -> Option<SceneNode> {
        *self.node.read().unwrap()
    }
}
