# Positioning 📍

Control how components are placed within the spatial scene.

---

## 🏗️ Usage
Positioning utilities allow you to break elements out of the normal document flow.

### API
| Method | Description |
| :--- | :--- |
| **`.relative()`** | Keeps element in flow but allows relative offsets. |
| **`.absolute()`** | Removes element from flow, relative to nearest parent. |
| **`.z(val)`** | Sets the stacking order (Z-index). |

---

## 🚀 Example
```rust
Div::new()
    .relative()
    .child(Badge::new("!").absolute().z(10))
```
