# Div 📦

The most basic, non-opinionated container in the Rupa Framework. Equivalent to an HTML `<div>` or a bare character block in TUI.

---

## 🏗️ Purpose
Use `Div` when you need a raw container to group elements or apply custom styles without the directional overhead of `VStack` or `HStack`.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.child(comp)`** | Adds a single child component (Internal Boxing). |
| **`.style(mod)`** | Applies a style modifier. |

---

## 🚀 Example
```rust
Div::new()
    .p(Scale::Md)
    .bg(Color::Zinc900)
    .child(Text::new("Inside a bare div"))
```
