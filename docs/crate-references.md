# Crate References 📚

This is the official registry of all the crates in the Rupa Framework monorepo, categorized by their structural tier.

---

## 🧱 1. Atomic Materials (Low-Level Units)

These crates handle single responsibilities and are the foundations of the framework.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-support`** | Atomic | The absolute foundation. Contains math (Vec2), ID generation, and common Errors. | `serde`, `thiserror` |
| **`rupa-signals`** | Atomic | The fine-grained reactivity engine. | `rupa-support`, `serde` |
| **`rupa-vnode`** | Atomic | The universal language. Contains the agnostic Virtual Tree structure **and core Style data models** (Color, OKLCH, Spacing). | `rupa-signals`, `taffy`, `serde` |
| **`rupa-store`** | Atomic | The Persistence System. Bridges reactivity to long-term storage (SQLite, WebStorage). | `rupa-signals`, `serde` |
| **`rupa-net`** | Atomic | The Network System. Handles asynchronous I/O and reactive data fetching. | `rupa-signals`, `futures` |
| **`rupa-motion`** | Atomic | The Animation Engine. High-performance VNode interpolation and spring physics. | `rupa-signals`, `rupa-vnode` |
| **`rupa-router`** | Atomic | The Navigation System. Agnostic URL mapping and view transitions. | `rupa-signals`, `rupa-vnode` |
| **`rupa-i18n`** | Atomic | The Voice. Multi-language dictionary management and cultural formatting. | `rupa-signals`, `serde_json` |
| **`rupa-assets`** | Atomic | The Warehouse. Pipeline for loading, decoding, and caching binary resources. | `rupa-signals`, `image` |
| **`rupa-a11y`** | Atomic | The Conscience. Bridge between VNode metadata and OS accessibility services. | `rupa-vnode`, `log` |
| **`rupa-auth`** | Atomic | The Identity System. Reactive authentication, RBAC, and Team management. | `rupa-signals`, `rupa-store` |
| **`rupa-forms`** | Atomic | The Form Engine. Reactive validation, form state tracking, and schemas. | `rupa-signals`, `serde` |
| **`rupa-context`**| Atomic | Dependency Injection. Scoped data sharing across the VNode tree. | `rupa-vnode`, `rupa-signals` |
| **`rupa-telemetry`**| Atomic | Observability. Metrics, structured logging, and performance profiling. | `rupa-core`, `log` |
| **`rupa-canvas`** | Atomic | Custom Graphics. Low-level WGPU access for artisan drawing and shaders. | `rupa-engine`, `wgpu` |

---

## 🛠️ 2. Composite Assembly Assemblies (High-Level Systems)

These crates assemble multiple atomic materials into functional, ready-to-use engines.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-core`** | Composite Assembly | The primary foundation. Integrates VNodes and base traits. | `rupa-signals`, `rupa-vnode`, `taffy` |
| **`rupa-ui`** | Composite Assembly | **The UI System.** Contains the **UI Component System** (elements) and **UI Utilities System** (Styling API). | `rupa-core`, `rupa-vnode` |
| **`rupa-engine`** | Composite Assembly | The Native Runtime. Handles hardware-accelerated rendering for Desktop and Terminal. | `rupa-core`, `wgpu`, `winit`, `crossterm` |
| **`rupa-server-core`** | Composite Assembly | The Backend & SSR Engine. Generates HTML/CSS. | `rupa-core`, `rupa-ui`, `axum`, `tokio` |
| **`rupa-web-core`** | Composite Assembly | The Web Frontend Engine. Handles DOM and WASM Hydration. | `rupa-core`, `rupa-ui`, `wasm-bindgen`, `web-sys` |
| **`rupa-mobile-core`** | Composite Assembly | The Mobile Integration bridge. Handles native platform interop (JNI for Android, C-FFI for iOS). | `rupa-core`, `rupa-engine`, `winit` |
| **`rupa-macros`** | Composite Assembly | Procedural stencils for code generation. | `syn`, `quote`, `proc-macro2` |
| **`rupa-docs`** | Composite Assembly | The official documentation app built using Rupa. | `rupa`, `rupa-ui` |

---

## 🛠️ 3. Developer Tooling

Crates designed to improve the development lifecycle.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-cli`** | Tooling | The Developer Interface. Scaffolding, dev-server, and multi-platform bundling. | `clap`, `tokio`, `serde` |
| **`rupa-test`** | Tooling | Quality Assurance. Headless VNode testing and interaction simulation. | `rupa-core`, `serde_json` |

---

## 🏪 4. Artisan Showrooms (Tier 3)

| Crate Name | Tier | Purpose | Key Features |
| :--- | :--- | :--- | :--- |
| **`rupa`** | Artisan | The universal showroom (All features). | `desktop`, `tui`, `ssr`, `web`, `mobile` |
| **`rupa-desktop`** | Artisan | Native GUI apps. | `gui`, `desktop` |
| **`rupa-web`** | Artisan | WASM/SPA frontend apps. | `web`, `wasm` |
| **`rupa-server`** | Artisan | SSR and Backend services. | `ssr`, `backend` |
| **`rupa-tui`** | Artisan | TUI and terminal tools. | `tui`, `terminal` |
| **`rupa-mobile`** | Artisan | Native Android/iOS entry point. | `mobile`, `gui` |
| **`rupa-fullstack`** | Artisan | Universal all-in-one Rupa bundle. | `web`, `ssr`, `desktop` |
| **`rupa-headless`** | Artisan | Logic-only services and automation. | `logic`, `persistence` |
| **`rupa-hybrid`** | Artisan | Rust to JS/TS interop bridge. | `wasm`, `interop` |

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

# Specialized Pure Frontend build (WASM)
rupa = { version = "0.1", features = ["web"] }

# Specialized Pure Backend build (SSR)
rupa = { version = "0.1", features = ["ssr"] }
```
