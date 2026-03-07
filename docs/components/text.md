# Component: Text 🔡

The `Text` component is the atomic semantic unit for displaying strings. It bridges the gap between raw reactive data and the Typography Engine.

---

## 🏗️ Architecture

- **Logic:** Manages a `Signal<String>` for the UTF-8 content.
- **VNode Mapping:** Renders as a `VNode::Text` node.
- **Styling:** Although a leaf node, it can be wrapped in an element to apply typographic styles (size, weight, color) which are then inherited or explicitly passed to the rendering engine.

---

## 🗝️ API & Usage

```rust
use rupa::prelude::*;

Text::new("Hello Rupa")
    .style(text_color(Color::Semantic("primary".into(), None)))
```

### Methods
- `.new(content)`: Creates a new Text component with a unique ID.

---

## 🌳 VNode Representation

When `render()` is called, a `Text` component produces:
```rust
VNode::Text("Hello Rupa".into())
```
