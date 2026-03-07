# Reactivity: Fine-Grained Updates ⚡

Rupa Framework avoids the common pitfall of "Re-rendering Everything." By using fine-grained reactivity, we ensure that the GPU and CPU only work on what has actually changed.

---

## 🧐 The Reactive Graph

Reactivity in the Rupa Framework is powered by `rupa-signals`.

1.  **Subscription:** When a component calls its `render()` method, any `Signal` accessed during that call registers the component as a dependency.
2.  **Invalidation:** When a `Signal`'s value is mutated, it notifies the framework that specific components are out-of-date.
3.  **Surgical Re-render:** Only the affected components re-execute their `render()` methods to produce new VNode sub-trees.
4.  **Patch Efficiency:** The diffing engine compares only the modified sub-trees against the previous state, generating minimal layout and paint patches.

---

## 🏛️ Integration with VNode Architecture

The **VNode Architecture** is the perfect vehicle for fine-grained updates:
- **State (Logic):** Mutates data wrapped in Signals.
- **Render (VNode):** Produces a declarative snapshot of the component's intent.
- **Reconciliation:** Translates VNode differences into precise hardware redraw requests.

## 🚀 Performance Benefits

- **Sub-millisecond Updates:** Even in deep UI trees, updating a single label is instantaneous.
- **Low Power Consumption:** Applications idle at nearly 0% CPU/GPU usage when no state changes occur.
- **Scalability:** The framework's performance scales linearly with the number of *changing* elements, not the total number of elements.
