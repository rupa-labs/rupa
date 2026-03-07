# The Styling API: Functional Utilities 🎨

The Styling API is the primary interface for defining the aesthetics of a Rupa Framework application. It follows a **Functional, Utility-First** approach inspired by modern atomic CSS frameworks.

---

## 🧠 Internal Anatomy

### 1. Style Modifiers
- **Mechanism:** Small functions (e.g., `p()`, `bg()`) that return an implementation of the `StyleModifier` trait.
- **Pure Functions:** Modifiers are stateless; they simply receive a mutable reference to a `Style` object and apply a specific transformation.

### 2. Composition (Tuples)
Rupa Framework implements `StyleModifier` for **Tuples**. This allows the framework to process multiple modifiers in a single pass, ensuring that layout invalidations only happen once after all styles are applied.

---

## 🗝️ Public API

### Standard Modifiers
| Category | Examples |
| :--- | :--- |
| **Spacing** | `p(16.0)`, `px(8.0)`, `m(4.0)` |
| **Sizing** | `w(100.0)`, `h_full()` |
| **Visual** | `bg(Color)`, `rounded(8.0)` |
| **Layout** | `flex()`, `flex_col()`, `gap(10.0)` |

### Chaining Interface (`Stylable`)
Every component that implements `Stylable` gains the `.style()` method:
```rust
Button::new("Save")
    .style((bg_primary(), p(12.0)))
```

---

## 🔄 Interaction Flow
- **Styling API -> VNode:** Utilities construct or mutate the `Style` object attached to a `VNode`.
- **Styling API -> Layout Engine:** Modifying layout-specific properties (like padding or flex direction) will trigger recalculations in Taffy during the Patch phase.
