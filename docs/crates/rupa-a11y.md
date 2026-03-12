# `rupa-a11y` ♿

**The Accessibility Port.** This crate provides **Atoms** for semantic accessibility and screen reader integration. It ensures that Rupa applications are inclusive by default across all platforms.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Prevents semantic ambiguity by enforcing standard ARIA-compatible roles.
    - **Sustain (S2)**: Reactive `Manager` automatically syncs the accessibility tree with visual changes.
    - **Scalable (S3)**: Decoupled `Bridge` port allows for specialized implementations (e.g., AccessKit for Desktop, Web A11y for WASM).

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Role`** | Semantic Identity. | Standard roles: Button, Link, Heading, Slider, etc. |
| **`Node`** | Accessible Data. | ARIA properties: Label, Value, Description, State. |
| **`Manager`** | Tree Orchestrator. | Maintains the global accessibility map reactively. |
| **`Bridge`** | The a11y Port. | Trait for communicating with OS-level assistive tech. |
| **`MockBridge`** | Testing Backend. | Captures screen reader announcements for verification. |

## 🚀 Usage

```rust
use rupa_a11y::{Node, Role, Manager, MockBridge};

// 1. Setup Manager with a bridge (usually at app init)
let manager = Manager::new().with_bridge(Arc::new(MockBridge::default()));

// 2. Define accessibility for a component
let btn_node = Node::new(Role::Button)
    .with_label("Submit Artisan Work")
    .with_value("Active");

manager.update_node("submit-btn", btn_node);

// 3. Make announcements
manager.bridge.unwrap().announce("Form submitted successfully");
```

## 🧪 Testing & Reliability
- **Verification**: `MockBridge` allows tests to assert that specific accessibility properties are being reported to the system.
- **TDD Support**: Headless tests can verify accessibility tree structure without a physical UI.
- **Agnostic**: The core Atom doesn't know about UIAutomation (Windows) or VoiceOver (macOS); it only provides the semantic port.
