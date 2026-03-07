# Module & Directory Standards 📂

To maintain a scalable and sustainable codebase, Rupa Framework strictly maps its **9-Layer Architecture** to specific directories in `src/`. All new modules must follow this placement guide.

---

## 🗺️ Layer-to-Directory Mapping

| Layer | Directory | Responsibility |
| :--- | :--- | :--- |
| **L9: Ecosystem** | `src/style/` | Design system tokens, OKLCH math, and Functional UI Utilities. |
| **L8: Composition** | `src/platform/mod.rs` | App bootstrap, main event loop orchestration. |
| **L7: Semantic** | `src/elements/` | High-level components (Button, Input, Navbar). |
| **L6: Primitives** | `src/primitives/` | Atomic building blocks (Div, Flex, Grid). |
| **L5: Architecture**| `src/core/` | Trait definitions (Component, Plugin) and Logic/View contracts. |
| **L4: Reactivity** | `src/utils/state.rs` | Signal, Memo, and the Reactive Graph. |
| **L3: Layout** | `src/layout/` | Taffy integration and geometric calculations. |
| **L2: Rendering** | `src/renderer/` | WGPU pipelines, batching, and text rendering. |
| **L1: Platform Integration** | `src/platform/` | Windowing (Winit) and native OS event dispatching. |

---

## 🧩 Component Internal Standard (L7)

Every semantic component in `src/elements/` must follow the **Logic & View** pattern. This can be implemented in a single file (for simple elements) or a directory (for complex ones).

### Standard Structure:
1.  **`ComponentLogic`**: Struct containing reactive signals, data, and event handlers. Pure UI logic.
2.  **`ComponentView`**: Struct containing `Style`, `NodeId`, and `dirty` flag. Rendering infrastructure.
3.  **`Component` (The Wrapper)**: The public-facing struct that implements `Component` and `Stylable` traits, acting as a bridge.

```rust
// Example: src/elements/my_component.rs
pub struct MyComponentLogic { ... } // Brain
pub struct MyComponentView { ... }  // Body
pub struct MyComponent { ... }      // Bridge
```

---

## 🎨 Styling & Utility Standard (L9)

The `src/style/` directory is divided into two distinct areas to ensure **DRY** and **SOC**:

### 1. `src/style/utilities/`
Contains "Passive Data". These are structs and enums that represent design tokens but do not perform actions.
- `color.rs` (OKLCH implementation)
- `spacing.rs` (Margin/Padding data)
- `scale.rs` (The 10-step unified scale)

### 2. `src/style/modifiers/`
Contains "Active Functions". These are the functional UI Utilities that users interact with.
- `utilities.rs` (Contains `p()`, `bg()`, `rounded()`, etc.)
- These functions must return types implementing the `StyleModifier` trait.

---

## 🛠️ General Rules

1.  **Deep Imports are Prohibited:** Users should only import from `rupa::prelude::*`. Internally, use `pub use` in `mod.rs` to flatten access.
2.  **Logic-View Boundary:** Logic structs must **never** import `wgpu` or `taffy`.
3.  **No Direct State Mutation:** Component state should always be wrapped in `Signal<T>` if it affects the UI output.
4.  **Utility First:** Prefer creating a generic modifier in L9 instead of hardcoding styles inside a component's View in L7.
