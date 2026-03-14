# Reactivity Port Architecture 🧬

The **Reactivity Port** is the heartbeat of the Rupa Framework. It provides the fine-grained reactivity engine that powers all state-driven updates in the ecosystem.

---

## 1. Core Primitives

The system is built on three fundamental reactive atoms:

### A. Signal
The primary source of truth. A `Signal<T>` holds a value and tracks a list of subscribers.
- **Identity**: Reactive state container.
- **Technical Role**: Publisher.

### B. Memo
A derived reactive value. A `Memo<T>` caches its result and only re-calculates when its upstream dependencies (Signals or other Memos) change.
- **Identity**: Computed state.
- **Technical Role**: Intermediate Publisher/Subscriber.

### C. Effect
A reactive side-effect. An `Effect` runs a closure automatically whenever its dependencies change.
- **Identity**: Side-effect orchestrator.
- **Technical Role**: Terminal Subscriber.

---

## 2. Technical Characteristics

- **Fine-Grained**: Updates are surgical. Changing a single signal only triggers the specific memos and effects that depend on it.
- **Thread-Safe**: Designed for multi-platform performance, allowing reactive state to be shared across threads safely.
- **Dependency Tracking**: Automatic and transparent. Artisans don't need to manually register dependencies.

---

## 3. Relationship with Other Systems

- **UI System**: Components consume Signals in their `render()` method to produce dynamic VNodes.
- **Storage Port**: Persistent signals synchronize their values with the `Storage Port` automatically.
- **Action Bus**: Handlers often update Signals to trigger UI updates as a result of an action.
