# Divider 📏

A visual separator used to distinguish between different content blocks.

---

## 🏗️ Purpose
`Divider` draws a thin line (Pixel in GUI, Unicode in TUI) between elements. It can be oriented horizontally or vertically.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.vertical()`** | Switches to vertical orientation. |
| **`.color(c)`** | Sets the divider color. |

---

## 🚀 Example
```rust
VStack::new()
    .child(Header::new())
    .child(Divider::new())
    .child(Body::new())
```
