# Element: Div 🧱

The `Div` is the most fundamental container in the Rupa Framework. It is a bare generic component used for grouping elements and applying styles without any opinionated defaults.

---

## 🏗️ Architecture

- **Logic:** Manages a collection of children components.
- **View:** Translates `Stylable` properties into a `VNode::Element`.
- **Default Behavior:** Renders as a standard `<div>` tag with `Style::default()`.

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

Div::new()
    .style((w(200.0), h(100.0), bg_primary()))
    .child(Text::new("Hello Rupa"))
```

### Methods
- `.new()`: Creates a new Div instance with a unique ID.
- `.child(component)`: Adds a child component to the container.

---

## 🌳 VNode Representation

When `render()` is called, a `Div` produces:
```rust
VNode::Element(VElement {
    tag: "div",
    style: /* current style */,
    attributes: /* current attributes */,
    children: /* rendered children */,
    key: Some(id),
})
```
