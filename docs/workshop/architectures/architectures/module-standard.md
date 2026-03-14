# Module & Workspace Standards 📂

To maintain a scalable and sustainable codebase, Rupa Framework strictly maps its architectural tiers to specific crates within the workspace. All new modules must follow this placement guide based on the **Atomic Materials & Composite Assemblies** model.

---

## 🗺️ Tier-to-Crate Mapping

| Tier | Workspace Crate | Responsibility |
| :--- | :--- | :--- |
| **Tier 1: Atomic Materials** | Foundational & Infrastructure | Virtual Tree, Reactivity, Math, Auth, Data, Net, etc. |
| **Tier 2: Composite Assemblies** | Technical Systems | Architecture, Engines (Native/Web/Mobile), UI System. |
| **Tier 3: Artisan Showrooms** | Target Facades | Desktop, Web, Server, TUI, Full-Stack, etc. |
| **Tooling** | Developer Tools | CLI and Testing utilities. |

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
