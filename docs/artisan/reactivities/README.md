# Reactivities & Events (The Pulse) 🧬

Reactivity is the heartbeat of the Rupa Framework. It transforms static data into a living, breathing user interface that responds instantly to state changes and user interactions.

In Rupa, we follow the **Fine-Grained** philosophy: when a value changes, the framework performs "surgical" updates, touching only the specific parts of the UI that absolutely must change. No full-tree re-renders, no unnecessary overhead.

---

## 🧬 Reactive Primitives
The building blocks of your application's state.

*   **[Signals](./signals.md)**: The primary source of truth. Observable state containers.
*   **[Memos](./memos.md)**: Intelligent, cached derivations of your state.
*   **[Effects](./effects.md)**: Automatic side-effects that bridge state to the real world.

---

## 🖱️ Interaction & Events
How users communicate their intent to your application.

*   **[Unified Input Model](./event-references.md)**: Understanding Pointers, Focus, and System events.
*   **[Semantic Intents](./event-references.md)**: Handling `on_submit`, `on_click`, and more.

---

## 📐 The Artisan's Way
Unlike traditional frameworks that use complex diffing algorithms for the entire state, Rupa allows you to define exactly **what** is reactive. This gives you absolute sovereignty over your application's performance.

```rust
let count = Signal::new(0);

// Only this Text node will ever update
Text::new(Memo::new(move || format!("Count: {}", count.get())))
```

*Learn more about the internal mechanics in the [**Workshop: Change Propagation**](../../workshop/architectures/change-propagation.md).*
