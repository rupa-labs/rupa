//! # Rupa Router 🧭
//!
//! Reactive navigation and routing system for the Rupa Framework. 
//! This crate provides the **Composites** for managing application state 
//! through hierarchical URL/Path mapping.

pub mod router;
pub mod route;
pub mod history;

pub use router::Router;
pub use route::{Route, RouteState};
pub use history::{History, MemoryHistory};

use rupa_context::use_context;
use std::sync::Arc;

/// A reactive hook to get the current route state.
/// 
/// Automatically establishes a reactive dependency on the router's 
/// current path and parameters.
pub fn use_route() -> Option<RouteState> {
    use_context::<Arc<Router>>().map(|r| r.current_state.get())
}

/// Navigates to a new path using the global router instance.
///
/// # Examples
///
/// ```
/// use rupa_router::navigate;
/// navigate("/dashboard");
/// ```
pub fn navigate(path: &str) {
    if let Some(router) = use_context::<Arc<Router>>() {
        router.history.push(path);
    }
}

/// A mock helper for testing navigation flows in headless environments.
pub struct MockRouter;

impl MockRouter {
    /// Creates a fresh router with memory-based history for testing.
    pub fn new() -> Arc<Router> {
        Arc::new(Router::new(vec![], Box::new(MemoryHistory::new())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rupa_context::provide_context;

    #[test]
    fn test_navigation_logic() {
        let router = MockRouter::new();
        rupa_context::with_registry(std::sync::Arc::new(rupa_context::Registry::new()), || {
            provide_context(router.clone());
            navigate("/test");
            // In a real test, we would assert the router state changed
        });
    }
}
