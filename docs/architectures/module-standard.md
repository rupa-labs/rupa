# Module & Workspace Standards 📂

To maintain a scalable and sustainable codebase, Rupa Framework strictly maps its architectural tiers to specific crates within the workspace. All new modules must follow this placement guide based on the **Atoms & Composites** model.

---

## 🗺️ Tier-to-Crate Mapping

| Tier | Workspace Crate | Responsibility |
| :--- | :--- | :--- |
| **Atom: Styling (Data)** | `rupa-vnode` | Passive Style Data (Design system tokens, OKLCH math, Scales). |
| **Atom: Reactivity** | `rupa-signals` | Signal, Memo, and the fine-grained Reactive Graph. |
| **Atom: VNode** | `rupa-vnode` | Agnostic Virtual Tree structure and serialization logic. |
| **Atom: Support** | `rupa-support` | Math (Vec2), ID generation, and common Errors. |
| **Composite: Architecture** | `rupa-core` | Trait definitions (Component), universal data structures, and VNode reconciliation. |
| **Composite: UI System** | `rupa-ui` | **UI Component System** (semantic elements) and **UI Utilities System** (Styling API). |
| **Composite: Engine** | `rupa-engine` | Native runtime, WGPU/TUI rendering, and Taffy layout integration. |
| **Composite: Mobile** | `rupa-mobile` | Mobile runner, lifecycle management, and touch events. |
| **Composite: Platform** | `rupa-server` / `rupa-client` | SSR and Web frontend engines. |

---

## 🧩 Component Internal Standard

Every semantic component in `crates/rupa-ui/src/elements/` must follow the **VNode** rendering pattern. This can be implemented in a single file (for simple elements) or a directory (for complex ones).

### Standard Structure:
1.  **Component Struct**: Contains reactive signals and data.
2.  **Implementation**: Implements `Component` and `Stylable` traits.
3.  **`render()` Method**: Produces a `VNode` tree that represents the semantic output of the component.

```rust
// Example: crates/rupa-ui/src/elements/my_component.rs
pub struct MyComponent { ... }

impl Component for MyComponent {
    fn render(&self) -> VNode { ... }
}
```

---

## 🎨 Styling & Utility Standard

The `crates/rupa-vnode/src/style/` directory is divided into distinct areas to ensure **DRY** and **SOC**:

### 1. Passive Data
Structs and enums that represent design tokens but do not perform actions.
- `color.rs` (OKLCH implementation)
- `spacing.rs` (Margin/Padding data)
- `types.rs` (Breakpoints, Scales, etc.)

### 2. Active Modifiers
Found in `crates/rupa-ui/src/style/modifiers/`. These are the functional UI Utilities that users interact with.
- Contains `p()`, `bg()`, `rounded()`, etc.
- These functions must return types implementing the `StyleModifier` trait.

---

## 🛠️ General Rules

1.  **Deep Imports are Prohibited:** Users should only import from `rupa::prelude::*`. Internally, use `pub use` in `mod.rs` to flatten access.
2.  **Agnosticism:** Component definitions must **never** import `wgpu` or `taffy` directly.
3.  **No Direct State Mutation:** Component state should always be wrapped in `Signal<T>` if it affects the UI output.
4.  **Utility First:** Prefer creating a generic modifier in `rupa-ui` instead of hardcoding styles inside a component's render method.
