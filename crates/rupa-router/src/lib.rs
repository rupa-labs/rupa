pub mod router;
pub mod route;
pub mod history;

pub use router::Router;
pub use route::{Route, RouteState};
pub use history::{History, MemoryHistory};

use rupa_context::use_context;
use std::sync::Arc;

/// A reactive hook to get the current route state.
pub fn use_route() -> Option<RouteState> {
    use_context::<Arc<Router>>().map(|r| r.current_state.get())
}

/// Navigates to a new path using the global router.
pub fn navigate(path: &str) {
    if let Some(router) = use_context::<Arc<Router>>() {
        router.history.push(path);
    }
}
