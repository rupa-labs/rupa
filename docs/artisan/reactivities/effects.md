# Effects ⚡

**Effects** are the primary way to bridge the reactive world of Rupa with the non-reactive "real world." They run an arbitrary side-effect whenever their dependencies change.

---

## 🏗️ Creation
An effect runs immediately once upon creation and then re-runs whenever any signal it accessed during execution is updated.

```rust
let status = Signal::new("Idle");

Effect::new(move || {
    // This code runs every time 'status' changes
    println!("System Status: {}", status.get());
});
```

---

## 🎯 Common Use Cases

-   **Logging & Telemetry**: Automatically record state changes.
-   **Manual API Synchronization**: Syncing reactive state with a non-Rupa library (e.g., a raw OpenGL call).
-   **Persistence**: Saving state to disk whenever it changes.
-   **Debugging**: Inspecting values during development.

---

## ⚠️ The Artisan's Warning
Effects should be used sparingly. Most of the time, what you need is a **Memo** (to derive data) or a **Component** (to manifest visuals). Only use an Effect when you need to trigger a **side-effect** that lives outside the Rupa UI tree.

> **Rule of Thumb**: If your Effect sets another Signal, you probably should have used a Memo instead.

---

## 🧬 Lifecycle
Effects are automatically cleaned up when the component that created them is unmounted, ensuring your application remains free of memory leaks and zombie processes.
