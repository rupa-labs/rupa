# Gap ↔️

Controls the space between child elements in a flex or grid container.

---

## 🏗️ Usage
`gap` ensures consistent separation between siblings without needing manual margins on every child.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.gap(val)`** | Sets the distance between children. | `.gap(Scale::Md)` |

---

## 🚀 Example
```rust
HStack::new()
    .gap(Scale::Sm)
    .child(Button::new("Cancel"))
    .child(Button::new("Save"))
```
