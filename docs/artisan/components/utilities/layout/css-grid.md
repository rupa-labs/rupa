# CSS Grid 🏁

Powerful 2D layout management for complex structures.

---

## 🏗️ Usage
Use grid utilities when you need precise control over both rows and columns simultaneously.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.grid()`** | Sets display to Grid. | `.grid()` |
| **`.cols(n)`** | Defines number of columns. | `.cols(3)` |
| **`.rows(n)`** | Defines number of rows. | `.rows(2)` |
| **`.col_span(n)`** | (Item) Spans across N columns. | `.col_span(2)` |

---

## 🚀 Example
```rust
Div::new()
    .grid()
    .cols(4)
    .gap(Scale::Md)
    .child(Sidebar.col_span(1))
    .child(Main.col_span(3))
```
