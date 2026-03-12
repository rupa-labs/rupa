# `rupa-vnode` 🧬

**The UI Contract.** Platform-agnostic Virtual UI Nodes and Style models that define the "DNA" of a Rupa application.

## 🛠️ Key Features

- **`VNode`**: The universal tree structure (Element, Text, Fragment, Component).
- **`VElement`**: A single UI element with Tag, Style, Attributes, and Event Handlers.
- **`Style`**: A rich, builder-pattern model for layout and visual properties.
- **`UIEvent`**: Agnostic event schema (Pointer, Key, Focus).

## 🚀 Usage

```rust
use rupa_vnode::{VNode, Style, Color};

// Create a styled element
let node = VNode::element("div")
    .with_style(Style::new()
        .bg(Color::hex(0x3366FF))
        .p(20.0)
        .rounded(8.0)
    )
    .with_child(VNode::text("Hello Rupa!"));

// Handle interaction
let button = VNode::element("button")
    .with_handler(|event| {
        println!("Clicked!");
    });
```
