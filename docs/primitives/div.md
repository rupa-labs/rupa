# Primitive: Div 🧱

The `Div` is the most fundamental container in the Rupa Framework. It is a logic-light component used for grouping elements and applying styles.

---

## 🏗️ Architecture

- **Logic:** Manages a collection of children components.
- **View:** Translates `Stylable` properties into a `VNode::Element`.
- **Default Behavior:** In the Native engine, it defaults to a Flex container with `FlexDirection::Column`. In the Web engine, it renders as a standard `<div>`.

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

Div::new()
    .style((w(200.0), h(100.0), bg_primary()))
    .child(Box::new(Text::new("Hello Rupa")))
```

### Methods
- `.new()`: Creates a new Div instance with a unique ID.
- `.child(component)`: Adds a boxed component to the children list.

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
