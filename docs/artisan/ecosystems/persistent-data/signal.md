# Module: Persistent Signal (`crates/rupa-store/src/signal.rs`) 🔄

This module provides the bridge between the reactive graph and the storage system.

---

## 🧩 `struct PersistentSignal<T>`
A reactive state container that automatically persists its value.

- **Initialization**: On creation, it attempts to load its initial value from the assigned `Store`.
- **Mutation**: When `set()` is called, it updates the in-memory value and asynchronously schedules a write to the `Store`.
- **Serialization**: Utilizes `Serde` to convert complex Rust types into storage-compatible formats (JSON/Binary).
