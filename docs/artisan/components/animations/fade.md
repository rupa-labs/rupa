# Fade Animations 🌫️

Smooth visibility transitions based on opacity changes.

---

## 🏗️ Purpose
`Fade` is the most common way to introduce or hide elements, providing a subtle and professional feel compared to abrupt appearance.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.fade_in()`** | Transitions opacity from 0.0 to 1.0. |
| **`.fade_out()`** | Transitions opacity from 1.0 to 0.0. |

---

## 🚀 Example
```rust
VStack::new()
    .child(Alert::new("Success!").fade_in())
```
