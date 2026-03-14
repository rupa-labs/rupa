# Module: Context Provider (`crates/rupa-context/src/provider.rs`) 📥

This module defines the mechanism for injecting data into the VNode tree.

---

## 🏗️ `struct Provider<T>`
A transparent VNode wrapper that holds a value of type `T`.

- **Tree Scope**: The provided value is accessible to all descendants in the virtual tree.
- **Reactivity**: If the value in the provider changes (if it's a Signal), it triggers updates in all consumers.
