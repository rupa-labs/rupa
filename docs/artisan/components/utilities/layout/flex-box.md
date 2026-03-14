# Flexbox 🧬

Directional layout management based on the Flexbox model.

---

## 🏗️ Usage
Define how children should be distributed and aligned within a container.

### API
| Method | Description |
| :--- | :--- |
| **`.flex_row()`** | Lays out children horizontally. |
| **`.flex_col()`** | Lays out children vertically. |
| **`.items_center()`**| Centers children on the cross-axis. |
| **`.justify_center()`**| Centers children on the main-axis. |
| **`.justify_between()`**| Distributes children with equal space between. |

---

## 🚀 Example
```rust
Div::new()
    .flex_row()
    .items_center()
    .child(Logo)
    .child(NavLinks)
```
