# Component: VStack ↕️

The `VStack` (Vertical Stack) is a layout component that arranges its children in a vertical column. It is a semantic wrapper around the `Flex` primitive with a pre-configured column direction.

---

## 🏗️ Architecture

- **Composition:** Composes a `Flex` primitive internally.
- **Pre-configuration:** Automatically sets `FlexDirection::Col` on its internal style.
- **VNode Mapping:** Renders as a `VNode::Element` with the tag "vstack" (or "flex" with column direction depending on the target).

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

VStack::new()
    .style(gap(10.0))
    .child(Box::new(Text::new("Item 1")))
    .child(Box::new(Text::new("Item 2")))
```

### Methods
- `.new()`: Initializes an empty vertical stack.
- `.child(component)`: Adds a child to the stack.

---

## 🌳 VNode Representation

When `render()` is called, a `VStack` produces:
```rust
VNode::Element(VElement {
    tag: "vstack",
    style: /* current style with FlexDirection::Col */,
    attributes: /* current attributes */,
    children: /* rendered children */,
    key: Some(id),
})
```
