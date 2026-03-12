# `rupa-router` 🔄

**The Navigation Engine.** Orchestrates application-wide view state and navigation history reactively.

## 🛠️ Key Features

- **`Router`**: The central manager for routing logic and active state.
- **`Route`**: Definition of navigable paths and their component factories.
- **`History` Port**: Hexagonal Port for interacting with the stack (supports Memory, Browser).
- **`use_route()`**: Reactive hook to access the current navigation state.
- **`navigate()`**: Global helper for programmatic navigation.

## 🚀 Usage

```rust
use rupa_router::{Router, Route, MemoryHistory, use_route, navigate};
use rupa_vnode::VNode;

// 1. Define routes
let routes = vec![
    Route::new("/", "Home", |_| VNode::text("Welcome Home")),
    Route::new("/about", "About", |_| VNode::text("About Us")),
];

// 2. Initialize Router with a History implementation
let history = Arc::new(MemoryHistory::new("/"));
let router = Router::new(routes, history);

// 3. Register in context
provide_context(router);

// 4. Use in components
let route = use_route();
if let Some(state) = route {
    println!("Current path: {}", state.path);
}

// 5. Navigate programmatically
navigate("/about");
```
