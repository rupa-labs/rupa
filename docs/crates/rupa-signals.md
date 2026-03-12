# `rupa-signals` 🧬

**The Heartbeat of Rupa.** This crate provides the foundational **Atoms** for fine-grained reactivity, serving as the reactive DNA for the entire framework.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Thread-local `Runtime` ensures state isolation and prevents cross-thread reactive leakage.
    - **Sustain (S2)**: High-fidelity Rustdoc with usage examples for every primitive.
    - **Scalable (S3)**: Zero-cost tracking overhead and lazy `Memo` evaluation.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Signal<T>`** | Reactive state container. | Thread-safe, Serde support, Atomic IDs. |
| **`Memo<T>`** | Derived reactive value. | Lazy evaluation, dependency tracking, Serde support. |
| **`Effect`** | Reactive side-effect. | Automatic re-run on dependency change. |
| **`batch`** | Batch multiple updates. | Prevents redundant effect execution. |
| **`untrack`** | Non-reactive read. | Reads signal value without establishing dependency. |

## 🚀 Usage

```rust
use rupa_signals::{Signal, Memo, Effect, batch, untrack};

// 1. Define reactive state (Atoms)
let count = Signal::new(10);
let name = Signal::new("Artisan".to_string());

// 2. Derive state (Lazy & Cached)
let greeting = Memo::new({
    let name = name.clone();
    move || format!("Hello, {}!", name.get())
});

// 3. Side Effects (Automatic)
Effect::new({
    let greeting = greeting.clone();
    let count = count.clone();
    move || {
        println!("{}, Count is: {}", greeting.get(), count.get());
    }
});

// 4. Batched Updates (Atomic)
batch(|| {
    count.set(20);
    name.set("Master".to_string());
}); // Effect runs only ONCE here

// 5. Untracked Read
let silent_val = untrack(|| count.get());
```

## 🧪 Testing & Reliability
- **TDD Driven**: Comprehensive test suite covering circular dependencies, batching, and `untrack` behavior.
- **Cross-Thread**: Verified safe state mutation across thread boundaries using `Arc` and `RwLock`.
- **Serializability**: Every Signal and Memo can be serialized/deserialized for persistence (see `rupa-store`).
