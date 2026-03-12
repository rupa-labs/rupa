# `rupa-queue` ⛓️

**The Orchestration Port.** This crate provides **Atoms** for background task management and systemic coordination. It enables asynchronous execution with fine-grained progress tracking.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Concurrency limiting prevents resource exhaustion and DoS-like scenarios.
    - **Sustain (S2)**: Reactive `Handle` API provides a single source of truth for task status.
    - **Scalable (S3)**: Task-agnostic queue design allows for heterogeneous background processing.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Task`** | The Background Port. | Contract for async operations with lifecycle hooks. |
| **`Queue`** | Task Orchestrator. | Concurrency control, priority support, reactive monitoring. |
| **`Handle`** | Reactive Tracker. | Signals for `Progress`, `Status` (Queued/Running/Done), and `Result`. |
| **`MockQueue`** | Testing Implementation. | Simplified queue for verifying task dispatch in tests. |

## 🚀 Usage

```rust
use rupa_queue::{Queue, Task, MockQueue};

// 1. Setup Queue (Usually provided via Context)
let queue = Queue::new(4); 

// 2. Dispatch a task (Implemented by Showroom)
queue.push(MyDataSyncTask::new());

// 3. Monitor state reactively
Effect::new({
    let queue = queue.clone();
    move || {
        println!("Active tasks: {}", queue.running_count().get());
    }
});
```

## 🧪 Testing & Reliability
- **Deterministic Execution**: `MockQueue` allows tests to verify that tasks are being correctly enqueued and completed without real concurrency noise.
- **Async Safety**: Verified safe handling of panicked tasks and cancellation signals.
- **TDD Support**: Built-in mechanisms for asserting task side-effects in headless environments.
