# `rupa-a11y` ♿

**The Accessibility Port.** Ensures the Rupa UI tree is accessible to OS-level assistive technologies in a platform-agnostic way.

## 🛠️ Key Features

- **`Role`**: Semantic roles based on ARIA standards (Button, Heading, TextInput, etc.).
- **`Node`**: Data model for accessibility properties (Label, Value, State, Bounds).
- **`Bridge`**: Port for implementation by platform adapters (e.g., AccessKit integration).
- **`Manager`**: Reactive orchestrator for dynamic accessibility tree updates.

## 🚀 Usage

```rust
use rupa_a11y::{Node, Role, Manager};

// 1. Define accessibility properties for an element
let mut a11y_node = Node::new(Role::Button)
    .with_label("Submit Form");

// 2. Report to the manager
let manager = use_context::<Manager>().unwrap();
manager.update_node("btn-123", a11y_node);
```
