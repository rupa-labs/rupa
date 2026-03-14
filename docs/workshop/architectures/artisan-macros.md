# Artisan Macros Architecture 🪄

**Artisan Macros** are Tier 2 procedural macros that provide ergonomic code generation and boilerplate reduction for the Rupa Framework.

---

## 1. Primary Macros

- **`#[derive(Component)]`**: Automatically implements the boilerplate for the `Component` trait.
- **`#[derive(Form)]`**: Generates form schemas and validation metadata from Rust structs.
- **`vnode!`**: A DSL macro for declaring VNode trees with HTML-like syntax.

---

## 2. Technical Role

- **Compile-Time Craft**: All logic is executed at compile time to ensure zero runtime overhead.
- **Semantic DX**: Focuses on improving the Developer Experience (DX) without sacrificing performance.
