# Reactivity: Fine-Grained Updates ⚡

Rupa Framework avoids the common pitfall of "Re-rendering Everything." By using fine-grained reactivity, we ensure that the GPU and CPU only work on what has actually changed.

---

## 🧐 The "Dirty" Flag Pattern

Every component in Rupa Framework (Layer 5) maintains a `dirty` flag (an `AtomicBool`).

1.  **Subscription:** When a component is created, it can subscribe to a Signal.
2.  **Invalidation:** When that Signal changes, the component calls `self.mark_dirty()`.
3.  **Layout Bypass:** During the Layout phase (Layer 3), if a component is NOT dirty, Taffy skips recalculating its subtree and reuses the previous coordinates.
4.  **Paint Efficiency:** If a component is not dirty and its global position hasn't changed, the renderer can potentially reuse previous draw commands (planned optimization).

---

## 🏛️ Integration with Logic & View

The **Logic & View** pattern is the perfect vehicle for fine-grained updates:
- **Logic:** Detects the data change.
- **View:** Reacts by setting its `dirty` flag.
- **Bridge:** Coordinates the redraw request to Layer 1.

## 🚀 Performance Benefits

- **Sub-millisecond Updates:** Even in deep UI trees, updating a single label is instantaneous.
- **Low Power Consumption:** Applications idle at nearly 0% CPU/GPU usage when no state changes occur.
- **Scalability:** The framework's performance scales linearly with the number of *changing* elements, not the total number of elements.
