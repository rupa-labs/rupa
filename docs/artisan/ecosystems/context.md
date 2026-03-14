# Module: Global State & Context (`crates/rupa-context`) 🧠

The `rupa-context` crate implements a Dependency Injection (DI) pattern for the VNode tree, allowing data to be shared across deeply nested components.

---

## 1. Core Philosophy: Tree-Scoped DI
Contexts allow "Portal-like" data access without passing props through every layer of the component tree.

## 2. Module Structure (1:1 Mapping)
- `provider.rs`: The VNode wrapper that injects data into a sub-tree.
- `consumer.rs`: Logic for retrieving context data from the nearest ancestor.
- `store.rs`: Central registry for global framework-level contexts.
