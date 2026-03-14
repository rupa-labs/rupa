# Container 🏗️

A structural unit designed to constrain and center content within a layout.

---

## 🏗️ Purpose
`Container` provides built-in max-width constraints and horizontal padding, making it the ideal wrapper for the main content areas of your application.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.child(comp)`** | Adds a component to the container. |
| **`.p(scale)`** | Adjusts the internal padding. |

---

## 🚀 Example
```rust
Container::new()
    .child(MyMainContent::new())
```
