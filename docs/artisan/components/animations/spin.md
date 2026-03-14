# Spin Animation 🔄

A continuous rotation effect, typically used for loading indicators.

---

## 🏗️ Purpose
`Spin` provides visual feedback that a process is active. It is the foundation for the `Spinner` component but can be applied to any material.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.spin()`** | Rotates the element 360 degrees infinitely. |
| **`.spin_reverse()`**| Rotates counter-clockwise. |

---

## 🚀 Example
```rust
Icon::new("loading-gear")
    .spin()
```
