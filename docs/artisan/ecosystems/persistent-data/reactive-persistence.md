# Reactivity: Persistent Lifecycle 🔄

This document explains the journey of data from memory mutation to persistent hardware storage.

---

## 🔁 Persistent Mutation Flow

### 1. Trigger (Signal Set)
A user calls `persistent_signal.set(new_value)`.
- This phase updates the value in RAM immediately to maintain UI responsiveness (Optimistic Update).

### 2. Reactive Propagation
The `PersistentSignal` triggers notifications to all subscribers (UI Components). VNode reconciliation begins.

### 3. Middleware: Serialization
The new value is passed through the **Serde** layer. Data is transformed into the format required by the backend (JSON, Binary, or SQL Parameters).

### 4. Hardware Commit (Write Phase)
The `PersistentSignal` asynchronously (or synchronously depending on config) calls `Store::write()`.
- **Native**: Opens a file handle or executes an SQL transaction.
- **Web**: Invokes the browser's `localStorage.setItem` or `IDBRequest` APIs.

---

## 🛡️ Data Integrity (3S Compliance)

- **Secure (S1)**: A failed write operation (e.g., disk full) will not corrupt the in-memory state. The system raises a `PersistenceError` that can be handled reactively.
- **Sustain (S2)**: Developers interact with a single API (`PersistentSignal`) regardless of the underlying database implementation.
- **Scalable (S3)**: Data writes can be *debounced* to prevent I/O overload during rapid state changes (e.g., moving a slider).
