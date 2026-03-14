# Show 👁️

A reactive logic component for conditional rendering.

---

## 🏗️ Purpose
`Show` uses a `Signal<bool>` to decide whether its children should be part of the VNode tree or not.

---

## 🚀 Example
```rust
Show::new(is_visible_signal)
    .child(SecretData::new())
```
