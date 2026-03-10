use std::sync::{RwLock, RwLockWriteGuard};
use std::sync::atomic::{AtomicBool, Ordering};
use rupa_vnode::Style;
use crate::scene::SceneNode;

/// The standardized infrastructure for any Component View.
/// Handles styling, geometric identity, and dirty-tracking.
pub struct ViewCore {
    pub style: RwLock<Style>,
    pub node: RwLock<Option<SceneNode>>,
    pub is_dirty: AtomicBool,
}

impl ViewCore {
    pub fn new() -> Self {
        Self {
            style: RwLock::new(Style::default()),
            node: RwLock::new(None),
            is_dirty: AtomicBool::new(true),
        }
    }

    pub fn mark_dirty(&self) {
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub fn clear_dirty(&self) {
        self.is_dirty.store(false, Ordering::SeqCst);
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty.load(Ordering::SeqCst)
    }

    pub fn style(&self) -> RwLockWriteGuard<'_, Style> {
        self.style.write().unwrap()
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
