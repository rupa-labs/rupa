# `rupa-fullstack` 🥞

**The Unified Showroom.** This crate acts as a meta-showroom that bundles `rupa-server` and `rupa-web` together to provide a seamless hydration experience.

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Manages the boundary where Server-Side Rendered HTML is taken over (hydrated) by the WebAssembly client logic.

## 🛠️ Key Features
- **Isomorphic Routing**: Define routes once, run them on the server and client.
- **State Hydration**: Safely transfer server-resolved signals into WASM memory.

## 🚀 Usage

```rust
use rupa_fullstack::prelude::*;

// Shared logic for both server and client
pub fn my_app() -> impl Component {
    Text::new("Hello Isomorphic World")
}
```
