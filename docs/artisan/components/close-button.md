# Close Button ✖️

A specialized interactive element designed for closing overlays or panels.

---

## 🏗️ Purpose
`CloseButton` provides a standard, high-contrast "X" button. It is pre-styled to be recognizable and consistent across all Rupa themes.

---

## 🚀 Example
```rust
Modal::new()
    .child(CloseButton::new().on_click(|_| close_modal()))
```
