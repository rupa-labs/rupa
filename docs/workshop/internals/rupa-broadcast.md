# `rupa-broadcast` 📡

**The Event Port.** This crate provides **Atoms** for decoupled, global reactive communication. It allows independent systems to talk to each other without direct coupling.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Channel-based message passing prevents shared-state corruption.
    - **Sustain (S2)**: Explicit `dispatch` and `subscribe` APIs clarify communication flows.
    - **Scalable (S3)**: Topic-based channels ensure that subscribers only process relevant messages.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Broadcaster<T>`** | The Event Port. | Contract for multi-subscriber message delivery. |
| **`Bus`** | Central Hub. | Orchestrates multiple typed channels. |
| **`MockBus<T>`** | Testing Implementation. | Records sent messages for verification in tests. |
| **`Channel<T>`** | Message Stream. | Thread-safe, async-capable message delivery. |

## 🚀 Usage

```rust
use rupa_broadcast::{Bus, Broadcaster, MockBus};

// 1. Get a channel for a specific event
let notifications = Bus::global().channel::<String>("alerts");

// 2. Subscribe to messages
notifications.subscribe(Arc::new(|msg| {
    println!("Received alert: {}", msg);
}));

// 3. Dispatch from anywhere
tokio::spawn(async move {
    notifications.dispatch("Low Disk Space".to_string()).await;
});
```

## 🧪 Testing & Reliability
- **Message Recording**: `MockBus` tracks all dispatched events, allowing tests to assert that specific notifications were triggered.
- **Async Safety**: Verified safe message delivery across `tokio` tasks and thread boundaries.
- **Decoupled**: Enables "System A" to trigger actions in "System B" without either knowing the other's implementation details.
