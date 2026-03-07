# Crate References 📚

This is the official registry of all the crates in the Rupa Framework monorepo, categorized by their structural tier.

---

## 🏗️ 1. The Atoms (Low-Level Building Blocks)

These crates handle single responsibilities and are the foundations of the framework.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-signals`** | Atom | The fine-grained reactivity engine and ID generation. | `serde` |
| **`rupa-core`** | Composite | The primary foundation. Contains basic traits, math, and events. | `rupa-signals`, `taffy`, `log` |

*Note: `rupa-core` is currently acting as a composite of several planned atoms like `rupa-styling`, `rupa-vnode`, and `rupa-layout-taffy`. These will be split as they mature.*

---

## 🧩 2. The Composites (High-Level Assemblies)

These crates assemble multiple atoms into functional, ready-to-use engines.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-ui`** | Composite | The Artisan Component Library. Contains `Button`, `VStack`, and standard primitives. | `rupa-core`, `taffy` |
| **`rupa-engine`** | Composite | The Native Runtime. Handles hardware-accelerated rendering for Desktop and Terminal. | `rupa-core`, `wgpu`, `winit`, `crossterm` |
| **`rupa-server`** | Composite | The Backend & SSR Engine. Generates HTML/CSS and integrates with server frameworks. | `rupa-core`, `rupa-ui`, `axum`, `tokio` |
| **`rupa-client`** | Composite | The Web Frontend Engine. Handles DOM manipulation and WASM Hydration. | `rupa-core`, `rupa-ui`, `wasm-bindgen`, `web-sys` |
| **`rupa-macros`** | Composite | Procedural macros for reducing boilerplate and boilerplate code generation. | `syn`, `quote`, `proc-macro2` |
| **`rupa-docs`** | Composite | The official documentation app built using the Rupa Framework. | `rupa`, `rupa-ui` |

---

## 🚀 3. The Facade (The Showroom)

| Crate Name | Tier | Purpose | Key Features |
| :--- | :--- | :--- | :--- |
| **`rupa`** | Facade | The unified entry point for developers. Re-exports composites based on feature flags. | `desktop`, `tui`, `ssr`, `web` |

---

## 🛠️ Usage Summary

Users typically depend on the `rupa` crate and enable the necessary features:

```toml
[dependencies]
# Standard Full-Stack Web build
rupa = { version = "0.1", features = ["web", "ssr"] }

# Specialized Native Desktop build
rupa = { version = "0.1", features = ["desktop"] }
```
