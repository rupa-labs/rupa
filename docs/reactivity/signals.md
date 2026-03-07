# Module: Signals & Memos (`state.rs`) 🧬

This module is the "Reactive Nucleus" of Rupa Framework. it provides the primitives required for fine-grained state management and automatic UI synchronization.

---

## 🧠 Internal Anatomy

### 1. Atomic Versioning
- **Mechanism:** Every `Signal` contains an `AtomicU64` version counter.
- **Responsibility:** Tracks how many times the value has changed. This serves as the foundation for the "Intelligent Memoization" system, allowing components to skip updates if the version remains constant.

### 2. Dependency Interface (`Readable<T>`)
Provides a unified trait implemented by both `Signal` and `Memo`. This ensures that components can consume reactive data without knowing if it's a raw value or a derived calculation.

---

## 🗝️ API Anatomy

### `struct Signal<T>`
- `.get()`: Thread-safe read.
- `.set()` / `.update()`: Mutations that increment the version and trigger global redraw notifications.

### `fn batch<F, R>(f: F)`
- **Mechanism:** Uses a thread-local counter to track the "Batch Nesting Level."
- **Responsibility:** Suppresses `request_redraw` calls until the counter returns to zero, allowing mass updates to be processed in a single frame.

---

## 🔄 Interaction Flow
- **L4 (Signal) -> L1 (Platform Integration):** Signals the platform shell to schedule a redraw.
- **L4 (Signal) -> L5 (Component):** Marks component views as `dirty` via connection callbacks.
