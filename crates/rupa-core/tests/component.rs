use rupa_core::Component;
use rupa_vnode::VNode;
use std::any::Any;
use std::sync::{Arc, RwLock};

struct ArtisanButton {
    prev_vnode: Arc<RwLock<Option<VNode>>>,
}

impl ArtisanButton {
    fn new() -> Self {
        Self {
            prev_vnode: Arc::new(RwLock::new(None)),
        }
    }
}

impl Component for ArtisanButton {
    fn render(&self) -> VNode {
        VNode::element("button")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn prev_vnode(&self) -> Arc<RwLock<Option<VNode>>> {
        self.prev_vnode.clone()
    }
}

#[test]
fn test_artisan_hooks() {
    let btn = ArtisanButton::new();
    // Testing default implementations (Concise names)
    btn.mount();
    btn.rendering();
    assert_eq!(btn.render(), VNode::element("button"));
    btn.rendered();
    btn.unmount();
}
