use rupa_vnode::VNode;
use std::collections::HashMap;
use std::sync::Arc;

pub type ComponentFactory = Arc<dyn Fn(RouteState) -> VNode + Send + Sync>;

/// Defines a single navigable path and its associated rendering logic.
#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub name: String,
    pub component: ComponentFactory,
}

impl Route {
    pub fn new(path: impl Into<String>, name: impl Into<String>, component: impl Fn(RouteState) -> VNode + Send + Sync + 'static) -> Self {
        Self {
            path: path.into(),
            name: name.into(),
            component: Arc::new(component),
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RouteState {
    pub params: HashMap<String, String>,
    pub path: String,
}
