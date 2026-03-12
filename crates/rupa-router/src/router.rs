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
            
            for route in &routes_inner {
                if let Some(params) = match_route(&route.path, &path) {
                    return RouteState {
                        path: path.clone(),
                        params,
                    };
                }
            }
            
            RouteState {
                path: path.clone(),
                params: std::collections::HashMap::new(),
            }
        });

        Arc::new(Self {
            routes,
            history,
            current_state,
        })
    }

    /// Navigates back to the previous entry in history.
    pub fn back(&self) {
        self.history.back();
    }
...
}

fn match_route(pattern: &str, path: &str) -> Option<std::collections::HashMap<String, String>> {
    let mut params = std::collections::HashMap::new();
    let p_parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let u_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if p_parts.len() != u_parts.clone().len() {
        return None;
    }

    for (p, u) in p_parts.into_iter().zip(u_parts) {
        if p.starts_with(':') {
            params.insert(p[1..].to_string(), u.to_string());
        } else if p != u {
            return None;
        }
    }

    Some(params)
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
