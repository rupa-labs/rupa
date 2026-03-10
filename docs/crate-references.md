# Crate References 📚

This is the official registry of all the crates in the Rupa Framework monorepo, categorized by their structural tier.

---

## 🏗️ 1. The Atoms (Low-Level Building Blocks)

These crates handle single responsibilities and are the foundations of the framework.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-support`** | Atom | The absolute foundation. Contains math (Vec2), ID generation, and common Errors. | `serde`, `thiserror` |
| **`rupa-signals`** | Atom | The fine-grained reactivity engine. | `rupa-support`, `serde` |
| **`rupa-vnode`** | Atom | The universal language. Contains the agnostic Virtual Tree structure **and core Style data models** (Color, OKLCH, Spacing). | `rupa-signals`, `taffy`, `serde` |
---

## 🧩 2. The Composites (High-Level Assemblies)

These crates assemble multiple atoms into functional, ready-to-use engines.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-core`** | Composite | The primary foundation. Integrates VNodes and base traits. | `rupa-signals`, `rupa-vnode`, `taffy` |
| **`rupa-ui`** | Composite | **The UI System.** Contains the **UI Component System** (elements) and **UI Utilities System** (Styling API). | `rupa-core`, `rupa-vnode` |
| **`rupa-engine`** | Composite | The Native Runtime. Handles hardware-accelerated rendering for Desktop and Terminal. | `rupa-core`, `wgpu`, `winit`, `crossterm` |
| **`rupa-server`** | Composite | The Backend & SSR Engine. Generates HTML/CSS. | `rupa-core`, `rupa-ui`, `axum`, `tokio` |
| **`rupa-client`** | Composite | The Web Frontend Engine. Handles DOM and WASM Hydration. | `rupa-core`, `rupa-ui`, `wasm-bindgen`, `web-sys` |
| **`rupa-mobile`** | Composite | The Mobile Integration bridge. Handles native interop and touch events. | `rupa-core`, `rupa-engine`, `winit` |
| **`rupa-macros`** | Composite | Procedural stencils for code generation. | `syn`, `quote`, `proc-macro2` |
| **`rupa-docs`** | Composite | The official documentation app built using Rupa. | `rupa`, `rupa-ui` |

---

## 🚀 3. The Facade (The Showroom)

| Crate Name | Tier | Purpose | Key Features |
| :--- | :--- | :--- | :--- |
| **`rupa`** | Facade | The unified entry point. Re-exports composites. | `desktop`, `tui`, `ssr`, `web`, `mobile` |

---

## 🛠️ Usage Summary

Users typically depend on the `rupa` crate and enable the necessary features:

```toml
[dependencies]
# Standard Full-Stack Web build
rupa = { version = "0.1", features = ["web", "ssr"] }

# Specialized Native Desktop build
rupa = { version = "0.1", features = ["desktop"] }

# Specialized Native Mobile build
rupa = { version = "0.1", features = ["mobile"] }
```
