# `rupa-vnode` 🌳

**The UI DNA of Rupa.** This crate defines the universal, platform-agnostic UI contract. It serves as the fundamental **Atom** for representing visual intent without hardware coupling.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Strict serializability boundaries ensure UI intent is predictable across system edges.
    - **Sustain (S2)**: A Fluent Builder API reduces cognitive load when constructing complex UI trees manually.
    - **Scalable (S3)**: Highly optimized tree structure designed for O(N) diffing performance.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`VNode`** | The Universal UI Node. | Enum supporting Elements, Text, Fragments, and Components. |
| **`VElement`** | A physical UI element. | Includes Tag, Style, Attributes, Handlers, and Motion. |
| **`VComponent`** | A lazy component placeholder. | Metadata-driven for late resolution by the Reconciler. |
| **`Style`** | Design System DNA. | Utility-first properties for Flexbox, Grid, and OKLCH colors. |

## 🚀 Usage (Fluent API)

```rust
use rupa_vnode::{VNode, Style, Color};

// 1. Construct a semantic UI tree using the Fluent Builder
let node = VNode::element("div")
    .with_key("main-container")
    .with_style(Style::default()
        .p(16.0)
        .bg(Color::oklch(0.8, 0.1, 240.0, 1.0)) // Aesthetic OKLCH
    )
    .with_child(VNode::element("span")
        .with_child(VNode::text("Artisan Craft"))
    );

// 2. Add attributes and interaction
let button = VNode::element("button")
    .with_attr("type", "submit")
    .with_handler(|_event| {
        println!("Interaction captured!");
    });
```

## 🧪 Testing & Reliability
- **TDD Driven**: Verified tree construction and nesting logic via automated builder tests.
- **Serializability**: Every `VNode` (excluding closures in handlers) implements `serde::Serialize` and `Deserialize`, allowing UI states to be transferred over the wire or saved to disk.
- **Hardware Agnostic**: Zero dependencies on `wgpu`, `winit`, or `web-sys`. This crate defines *what* the UI is, not *how* it is rendered.
