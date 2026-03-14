# `rupa-test` 🧪

**The Quality Atom.** This crate provides **Atoms** for headless testing, interaction simulation, and structural validation. it ensures that the framework and its applications fulfill the mandatory **TDD Mandate**.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Sandbox isolation ensures that tests do not leak state or side-effects.
    - **Sustain (S2)**: Semantic assertion API (e.g., `assert_element`) clarifies test intent.
    - **Scalable (S3)**: Lightweight headless runner allows for thousands of tests to run in seconds.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Tester`** | Headless Runner. | Manages a virtual application lifecycle for testing. |
| **`Snapshot`** | Tree Assertion. | Deep structural comparison of VNode trees. |
| **`MockInteraction`**| Event Simulation. | Simulates clicks, keys, and focus without a physical OS. |
| **`setup()`** | Quick Start. | Bootstraps a fresh test environment with default registries. |

## 🚀 Usage

```rust
use rupa_test::{setup, Snapshot};

#[test]
fn test_component_rendering() {
    // 1. Setup headless environment
    let t = setup();
    
    // 2. Render a component sub-tree
    let node = MyComponent::new().render();
    
    // 3. Assert structure
    Snapshot::assert_element(&node, "button", 1);
    Snapshot::assert_text_contains(&node, "Click Me");
}

#[test]
fn test_interaction() {
    let interaction = MockInteraction::new();
    interaction.fire_event();
    assert_eq!(interaction.events_fired.get(), 1);
}
```

## 🧪 Testing & Reliability
- **Self-Testing**: The testing Atoms are used to test all other Tier 1 crates, ensuring a circular guarantee of quality.
- **Snapshot Logic**: Verified to correctly identify minimal differences in complex, nested VNode trees.
- **CI Ready**: 100% focused on headless execution for seamless integration into automated pipelines.
