# Reactivity: Signals, Memos, and Effects 🧪

Rupa Framework uses a **Fine-Grained Reactivity** system powered by the `rupa-signals` atomic material. This system ensures that UI updates are surgical and only occur when the underlying data actually changes.

---

## 1. Core Concepts

### 1.1 Signals (`Signal<T>`)
Signals are the primary source of truth. They hold a value and notify subscribers when that value is updated.
-   **Getter**: `signal.get()` tracks the dependency if called within a reactive context (inside an Effect or Memo).
-   **Setter**: `signal.set(value)` triggers updates to all subscribers.

### 1.2 Memos (`Memo<T>`)
Memos are derived reactive values. They compute a value based on other signals or memos and cache the result.
-   **Lazy Evaluation**: The computation only runs when a dependency changes.
-   **Efficiency**: If the dependencies haven't changed, the cached value is returned immediately.

### 1.3 Effects (`Effect`)
Effects are side-effects that run when their dependencies change. In Rupa Framework, the rendering engine is essentially a giant Effect that reacts to VNode changes.

---

## 2. Technical Implementation: Dependency Tracking

`rupa-signals` implements automatic dependency tracking using a **Thread-Local Reactive Context**.

### How it works:
1.  **Context Stack**: When an `Effect` or `Memo` starts its computation, it pushes itself onto a global (thread-local) stack.
2.  **Observation**: When `signal.get()` is called, it checks the top of the stack. If a reactive context exists, the signal adds that context as a subscriber.
3.  **Notification**: When `signal.set()` is called, it iterates through its list of subscribers and tells them to re-run.
4.  **Cleanup**: Subscribers automatically clear their previous dependencies before re-running to handle conditional logic (e.g., `if count.get() > 5 { other.get() }`).

---

## 3. Thread Safety & Concurrency

Unlike many JavaScript frameworks, Rupa's reactivity is designed for Rust's safety guarantees:
-   **Synchronization**: Internal state uses `Arc<RwLock<T>>` to allow safe multi-threaded access.
-   **Async Readiness**: The system is designed to eventually support async effects, allowing data fetching to trigger UI updates without blocking the main thread.

---

## 4. Usage Example

```rust
let count = Signal::new(0);

// This memo will only re-calculate when 'count' changes.
let is_even = Memo::new({
    let count = count.clone();
    move || count.get() % 2 == 0
});

// This effect runs once immediately, and again whenever 'is_even' changes.
Effect::new(move || {
    println!("Is even? {}", is_even.get());
});

count.set(2); // Triggers re-run
```
