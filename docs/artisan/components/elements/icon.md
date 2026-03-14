# Icon 🏷️

A semantic material for rendering vector icons.

---

## 🏗️ Purpose
`Icon` provides a standardized way to use iconography across your application. It abstracts away the specific SVG paths or font-glyph details.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.new(name)`** | Initializes the icon by its unique name. |
| **`.size(scale)`** | Scales the icon semantically. |

---

## 🚀 Example
```rust
Button::new("Settings")
    .child(Icon::new("settings-cog"))
```
