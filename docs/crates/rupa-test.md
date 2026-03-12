# `rupa-test` 🧪

**The Testing Material.** Utilities for headless component testing and reactive logic validation, fulfilling the TDD mandate.

## 🛠️ Key Features

- **`Tester`**: A headless environment that isolates context and signals.
- **`Snapshot`**: Structural assertions for VNode trees.
- **`setup()`**: Quick bootstrap for unit tests.

## 🚀 Usage

```rust
use rupa_test::{setup, Snapshot};
use rupa_signals::Signal;

#[test]
fn test_reactive_logic() {
    let t = setup();
    let count = Signal::new(0);
    
    // Assert signal value after action
    t.assert_signal(count.clone(), |s| s.set(10), 10);
}

#[test]
fn test_vnode_structure() {
    let node = VNode::element("div")
        .with_child(VNode::text("Hello"));
        
    Snapshot::assert_element(&node, "div", 1);
}
```
