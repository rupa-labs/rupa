/// Defines a single navigable path and its associated metadata.
pub struct Route {
    pub path: String,
    pub name: String,
}

pub struct RouteState {
    pub params: std::collections::HashMap<String, String>,
}
