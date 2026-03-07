# Crate References 📚

This is the official registry of all the crates in the Rupa Framework monorepo, categorized by their structural tier.

---

## 🏗️ 1. The Atoms (Low-Level Building Blocks)

These crates handle single responsibilities and are the foundations of the framework.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-signals`** | Atom | The fine-grained reactivity engine and ID generation. | `serde` |
| **`rupa-styling`** | Atom | The visual DNA, OKLCH color math, and unified design tokens. | `rupa-signals`, `taffy`, `once_cell` |
| **`rupa-vnode`** | Atom | The agnostic Virtual Tree structure used as a universal interface. | `rupa-styling`, `serde`, `serde_json` |

---

## 🧩 2. The Composites (High-Level Assemblies)

These crates assemble multiple atoms into functional, ready-to-use engines.

| Crate Name | Tier | Purpose | Key Dependencies |
| :--- | :--- | :--- | :--- |
| **`rupa-core`** | Composite | The primary foundation. Integrates VNodes and base traits. | `rupa-signals`, `rupa-styling`, `rupa-vnode`, `taffy` |
| **`rupa-ui`** | Composite | The Artisan Component Library. Contains `Button`, `Text`, `VStack`. | `rupa-core` |
| **`rupa-engine`** | Composite | The Native Runtime. Handles GPU (WGPU) and Terminal (TUI). | `rupa-core`, `wgpu`, `winit`, `crossterm` |
| **`rupa-server`** | Composite | The Backend & SSR Engine. Generates HTML/CSS. | `rupa-core`, `rupa-ui`, `axum`, `tokio` |
| **`rupa-client`** | Composite | The Web Frontend Engine. Handles DOM and WASM Hydration. | `rupa-core`, `rupa-ui`, `wasm-bindgen`, `web-sys` |
| **`rupa-macros`** | Composite | Procedural stencils for code generation. | `syn`, `quote`, `proc-macro2` |
| **`rupa-docs`** | Composite | The official documentation app built using Rupa. | `rupa`, `rupa-ui` |

---

## 🚀 3. The Facade (The Showroom)

| Crate Name | Tier | Purpose | Key Features |
| :--- | :--- | :--- | :--- |
| **`rupa`** | Facade | The unified entry point. Re-exports composites. | `desktop`, `tui`, `ssr`, `web` |
