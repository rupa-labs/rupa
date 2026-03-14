# Action Bus Architecture 🚌

The **Action Bus** is the primary asynchronous messaging system of the Rupa Framework. It follows the **Observer Pattern** and provides a decoupled way for components and systems to communicate through **Intents (Actions)** without knowing who will execute them.

---

## 1. Core Concepts

### A. The Action (The Message ✉️)
An `Action` is a serializable, thread-safe data structure that represents a specific intent or command.
- **Requirement**: Must implement `Serialize` and `Deserialize` to enable cross-thread or cross-process communication.
- **Identity**: Every action has a unique `id` for tracing and debugging.

### B. The Handler (The Executor 🛠️)
A `Handler` is a specialized system that knows how to execute a specific type of `Action`. 
- Handlers are usually registered by the **Plugin System** or the **Platform Runner**.
- Handlers are **stateless**; they receive an action and perform a side-effect (e.g., saving to disk, sending a network request).

### C. The Bus (The Dispatcher ⚡)
The `Bus` is a reactive channel that manages the distribution of actions to their respective handlers. It supports:
- **Broadcasting**: One action can be heard by multiple observers.
- **Type-Safety**: Rust's type system ensures that handlers only receive the specific action types they are subscribed to.

---

## 2. Why use the Action Bus?

1. **Decoupling**: A `Button` component in Tier 2 can dispatch a `SaveDocument` action without knowing that a `DatabasePlugin` in Tier 3 will eventually handle it.
2. **Telemetry**: Because every action is a discrete object, the framework can automatically log, trace, and measure the performance of every user intent.
3. **Replayability**: Actions can be recorded and replayed, enabling features like "Undo/Redo" or automated bug reproduction.

---

## 3. Standard Action Flow

1. **Dispatch**: A component calls `Bus::dispatch(MyAction { .. })`.
2. **Channeling**: The Bus identifies the action type and sends it to the correct reactive channel.
3. **Execution**: The subscribed `Handler` receives the action and performs the logic.
4. **Reactive Feedback**: If the handler changes some state, it updates a **Signal**, which then triggers the **Reconciler** to update the UI.
