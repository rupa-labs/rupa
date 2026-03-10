use std::sync::{RwLock, RwLockWriteGuard, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use rupa_vnode::{Style, VNode};
use crate::scene::SceneNode;
use rupa_signals::Subscriber;

static REDRAW_CALLBACK: RwLock<Option<Box<dyn Fn() + Send + Sync>>> = RwLock::new(None);

pub fn set_redraw_callback(f: impl Fn() + Send + Sync + 'static) {
    *REDRAW_CALLBACK.write().unwrap() = Some(Box::new(f));
}

pub fn request_redraw() {
    if let Some(ref f) = *REDRAW_CALLBACK.read().unwrap() {
        (f)();
    }
}

/// The standardized infrastructure for any Component View.
/// Handles styling, geometric identity, and dirty-tracking.
pub struct ViewCore {
    pub style: Arc<RwLock<Style>>,
    pub node: Arc<RwLock<Option<SceneNode>>>,
    pub is_dirty: Arc<AtomicBool>,
    pub prev_vnode: Arc<RwLock<VNode>>,
}

impl Clone for ViewCore {
    fn clone(&self) -> Self {
        Self {
            style: self.style.clone(),
            node: self.node.clone(),
            is_dirty: self.is_dirty.clone(),
            prev_vnode: self.prev_vnode.clone(),
        }
    }
}

impl Subscriber for ViewCore {
    fn notify(&self) {
        self.mark_dirty();
        request_redraw();
    }
}

impl ViewCore {
    pub fn new() -> Self {
        Self {
            style: Arc::new(RwLock::new(Style::default())),
            node: Arc::new(RwLock::new(None)),
            is_dirty: Arc::new(AtomicBool::new(true)),
            prev_vnode: Arc::new(RwLock::new(VNode::Empty)),
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

    pub fn set_prev_vnode(&self, node: VNode) {
        *self.prev_vnode.write().unwrap() = node;
    }

    pub fn get_prev_vnode(&self) -> VNode {
        self.prev_vnode.read().unwrap().clone()
    }
}
