# Component: HStack ↔️

The `HStack` (Horizontal Stack) is a layout component that arranges its children in a horizontal row. It is a semantic wrapper around the `Flex` primitive with a pre-configured row direction.

---

## 🏗️ Architecture

- **Composition:** Composes a `Flex` primitive internally.
- **Pre-configuration:** Automatically sets `FlexDirection::Row` on its internal style.
- **VNode Mapping:** Renders as a `VNode::Element` with the tag "hstack" (or "flex" with row direction depending on the target).

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

HStack::new()
    .style(gap(16.0))
    .child(Box::new(Button::new("Cancel")))
    .child(Box::new(Button::new("Confirm")))
```

### Methods
- `.new()`: Initializes an empty horizontal stack.
- `.child(component)`: Adds a child to the stack.

---

## 🌳 VNode Representation

When `render()` is called, a `HStack` produces:
```rust
VNode::Element(VElement {
    tag: "hstack",
    style: /* current style with FlexDirection::Row */,
    attributes: /* current attributes */,
    children: /* rendered children */,
    key: Some(id),
})
```
