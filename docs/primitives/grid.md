# Primitive: Grid 🧱

The `Grid` primitive is a two-dimensional layout container. It allows for complex layouts with rows and columns.

---

## 🏗️ Architecture

- **Logic:** Manages a collection of children components.
- **View:** Configures the `Style` with `Display::Grid` and produces a `VNode::Element`.

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

Grid::new()
    .style((
        grid_cols(vec!["1fr".into(), "1fr".into()]),
        gap(20.0),
        bg_surface()
    ))
    .child(Box::new(Text::new("Cell 1")))
    .child(Box::new(Text::new("Cell 2")))
```

---

## 🌳 VNode Representation

When `render()` is called, a `Grid` produces:
```rust
VNode::Element(VElement {
    tag: "grid",
    style: /* current style with Display::Grid */,
    attributes: /* current attributes */,
    children: /* rendered children */,
    key: Some(id),
})
```
