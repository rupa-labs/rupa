# Primitive: Flex 🧱

The `Flex` primitive is a one-dimensional layout container. It is used to align and distribute space among items in a row or column.

---

## 🏗️ Architecture

- **Logic:** Manages a collection of children components.
- **View:** Configures the `Style` with `Display::Flex` and produces a `VNode::Element`.
- **Relationship:** Higher-level layouts like `VStack` and `HStack` are specialized wrappers around the `Flex` primitive.

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

Flex::new()
    .style((flex_row(), justify_between(), items_center(), gap(10.0)))
    .child(Box::new(Text::new("Left")))
    .child(Box::new(Text::new("Right")))
```

---

## 🌳 VNode Representation

When `render()` is called, a `Flex` produces:
```rust
VNode::Element(VElement {
    tag: "flex", // Optimized for Rupa's native engine
    style: /* current style with Display::Flex */,
    attributes: /* current attributes */,
    children: /* rendered children */,
    key: Some(id),
})
```
