# `rupa-core` 🧠

**The Reconcilation Engine.** The heart of the framework that manages the UI lifecycle and tree updates.

## 🛠️ Key Features

- **`reconcile()`**: Sophisticated diffing algorithm for VNode trees.
- **`Patch`**: Standardized instructions for renderers to apply changes.
- **`Component`**: VNode-first trait for all UI elements.
- **`Action Bus`**: Reactive command pattern for decoupled state mutations.
- **`Renderer` Port**: Driven Port for hardware-specific implementations.

## 🚀 Usage

```rust
use rupa_core::{reconcile, Component, VNode};

// 1. Define a custom component
struct MyComp;
impl Component for MyComp {
    fn id(&self) -> &str { "my-comp" }
    fn render(&self) -> VNode { VNode::text("Hello World") }
}

// 2. Perform reconciliation
let patches = reconcile(&old_vnode, &new_vnode, None, 0);

// 3. Apply patches via a Renderer
renderer.render_patch(patches);
```
