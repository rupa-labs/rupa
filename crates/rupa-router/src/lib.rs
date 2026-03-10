pub mod router;
pub mod route;
pub mod history;

pub use router::Router;
pub use route::Route;

/// A reactive hook to get the current route state.
pub fn use_route() -> route::RouteState {
    todo!("use_route not yet implemented")
}
