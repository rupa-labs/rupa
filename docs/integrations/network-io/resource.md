# Module: Reactive Resource (`crates/rupa-net/src/resource.rs`) 🔄

This module provides the reactive primitive for asynchronous data fetching.

---

## 🧩 `struct Resource<T>`
A reactive state machine that tracks the status of an async request.

- **States**: `Uninitialized`, `Loading`, `Ready(T)`, `Error(E)`.
- **Automatic Updates**: When the async task completes, it updates its internal signal, triggering a VNode re-render for any component using the data.
