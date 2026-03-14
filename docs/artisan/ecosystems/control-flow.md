# Module: Control Flow (`control_flow.rs`) 🔀

Control Flow components allow developers to describe dynamic UI structures (conditionals and loops) using a declarative, reactive approach.

---

## 🧠 Internal Anatomy

### 1. Reactive Branching (`Show`)
- **Logic:** Tracks a `Signal<bool>`.
- **View:** Performs **Conditional Layout**. If the signal is false, it returns an empty node to Taffy, effectively removing the component from the visual and hit-test tree without destroying its state.

### 2. Reactive Lists (`ForEach`)
- **Logic:** Tracks a `Signal<Vec<T>>`.
- **Responsibility:** Manages the lifecycle of a collection of components. It ensures that when the list changes, only the necessary children are updated or re-laid out (Fine-Grained Updates).

---

## 🗝️ Public API

### `struct Show`
- `Show::new(when: Signal<bool>, content: impl Component)`: Conditionally renders the content.

### `struct ForEach`
- `ForEach::new(data: Signal<Vec<T>>, factory: impl Fn(T) -> Component)`: Maps data to UI elements.

---

## 🚀 Performance
By integrating directly with the **Layout Engine**, Control Flow components ensure that hidden branches consume zero GPU resources and zero layout calculation time.
