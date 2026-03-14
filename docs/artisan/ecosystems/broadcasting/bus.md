# Module: Broadcast Bus (`crates/rupa-broadcast/src/bus.rs`) 🚌

This module defines the central hub for the notification system.

---

## 🏗️ `struct BroadcastBus`
A thread-safe, global registry for all event channels.

- **Channel Management**: Allows dynamic creation and retrieval of named `Channels`.
- **Global Access**: Provided via a `Lazy` static or injected through `rupa-context`.
- **Thread Safety**: Uses `Arc<RwLock>` to allow simultaneous publishing and subscribing across threads.
