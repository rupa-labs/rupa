# Broadcast Port Architecture 📡

The **Broadcast Port** is the foundational infrastructure for global, decoupled communication within the Rupa Framework.

---

## 1. Core Units

- **Broadcaster Trait**: The primary interface for dispatching and receiving messages.
- **Reactive Channels**: Typed channels that ensure messages reach the correct subscribers.
- **Bus Orchestrator**: Manages multiple concurrent broadcast channels.

---

## 2. Technical Context

- **Decoupling**: Allows systems to communicate without direct dependencies.
- **Async First**: Messages are dispatched asynchronously to prevent blocking the render loop.
- **Thread-Safe**: Safe for use in multi-threaded environments.
