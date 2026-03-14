# Module: Event Channel (`crates/rupa-broadcast/src/channel.rs`) 📡

This module defines the individual streams of events.

---

## 🏗️ `struct Channel<T>`
A specialized stream for a specific event type `T`.

- **Publishing**: Method to send a message to all active subscribers.
- **Subscription**: Allows adding new listeners that implement the `Subscriber` trait.
- **Weak Referencing**: Uses weak pointers to subscribers to prevent memory leaks and circular dependencies.
