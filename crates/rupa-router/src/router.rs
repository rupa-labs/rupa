use rupa_signals::{Memo, Readable};
use crate::route::{Route, RouteState};
use crate::history::History;
use rupa_vnode::VNode;
use std::sync::Arc;

/// The central orchestrator for application navigation.
pub struct Router {
    pub routes: Vec<Route>,
    pub history: Arc<dyn History>,
    pub current_state: Memo<RouteState>,
}

impl Router {
    pub fn new(routes: Vec<Route>, history: Arc<dyn History>) -> Arc<Self> {
        let history_clone = history.clone();
        let routes_inner = routes.clone();
        
        let current_state = Memo::new(move || {
            let path = history_clone.current();
            let state = RouteState {
                path: path.clone(),
                params: std::collections::HashMap::new(),
            };

            for route in &routes_inner {
                if route.path == path {
                    return state;
                }
            }
            state
        });

        Arc::new(Self {
            routes,
            history,
            current_state,
        })
    }

    pub fn render(&self) -> VNode {
        let state = self.current_state.get();
        for route in &self.routes {
            if route.path == state.path {
                return (route.component)(state);
            }
        }
        VNode::Empty
    }
}
