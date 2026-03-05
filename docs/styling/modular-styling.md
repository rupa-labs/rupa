# Modular & Functional Styling

Rupaui features a unique **Modular Styling API** that combines the power of atomic utility functions with the safety of Rust's type system. This allows you to compose styles using tuples, closures, or full objects.

## 📐 The `StyleModifier` Trait

Any type that implements `StyleModifier` can be passed to a component's `.style()` method. 

### 1. Functional Atomic Utilities
Rupaui provides standalone functions for common styles. You can group them using **Tuples**.

```rust
use rupaui::utils::{p, bg, w, h, rounded};

Div::new().style((
    p(10.0),
    bg("indigo"),
    w(200.0),
    h(100.0),
    rounded(8.0)
));
```

---

## 🏗 Advanced Composition

### 2. Full Style Objects
You can still use the traditional `Style` object for complex or shared logic.

```rust
let card_base = Style::new().p(20.0).bg("zinc-900");

Div::new().style(card_base);
```

### 3. Closure Modifiers (Dynamic Logic)
For highly dynamic styling, you can pass a closure that modifies the style in-place.

```rust
Div::new().style(|s: &mut Style| {
    if user_is_active {
        s.background.color = Some("green".into());
    }
});
```

---

## ⚡ Performance Benefits
This modular approach ensures that style calculations are:
- **Zero-Allocation**: Tuples and atomic functions are resolved at compile-time or stack-allocated.
- **DOD-Friendly**: Modifiers update flat data structures, maximizing cache efficiency.
- **Wasm-Optimized**: Reduces the number of function calls compared to deep object nesting.
