# `rupa-broadcast` 📢

**The Global Event Bus.** Reactive multi-channel communication for decoupled systems.

## 🛠️ Key Features

- **`Broadcaster<T>`**: Port for dispatching and listening to typed messages.
- **`Bus`**: Central orchestrator managing multiple typed channels.
- **`Channel<T>`**: Thread-safe implementation of a message stream.

## 🚀 Usage

```rust
use rupa_broadcast::Bus;

// 1. Get a channel for a specific event type
let system_bus = Bus::global();
let notifications = system_bus.channel::<String>("notifications");

// 2. Subscribe to events
notifications.subscribe(Arc::new(|msg| {
    println!("New Notification: {}", msg);
}));

// 3. Dispatch from anywhere
tokio::spawn(async move {
    notifications.dispatch("System Update Complete".to_string()).await;
});
```
