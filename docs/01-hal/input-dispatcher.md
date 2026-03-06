# Module: Input Dispatcher (`dispatcher.rs`) 🎯

The Input Dispatcher is the sensory processing unit of Rupaui. It connects standardized input signals to the correct UI component via recursive spatial search.

---

## 🧠 Internal Anatomy

### 1. The Hit-Test Pipeline
The dispatcher does not calculate geometry itself. Instead, it queries the **Geometric Scene Layer (L3)**:
1. Receive `InputEvent`.
2. Ask `SceneCore::find_target(coordinate)`.
3. Receive a `HitDiscovery` (either `Missed` or `Found`).

### 2. Event Propagation (Bubbling)
When a target is `Found`, the dispatcher:
1. Creates a `UIEvent` context.
2. Traverses the component path from the leaf up to the root.
3. Calls the respective semantic hook (e.g., `on_click`).
4. Checks for `event.consumed` at each step to stop propagation.

---

## 🗝️ API Anatomy

### `struct UIEvent`
The rich context passed to components:
- `local_pos`: Pointer coordinate relative to the component's top-left corner.
- `modifiers`: The state of Shift/Ctrl/Alt during the event.
- `consume()`: Method to stop event bubbling.

---

## 🔄 Interaction Flow
- **L1 (Runner) -> L1 (Dispatcher):** Feeds normalized events.
- **L1 (Dispatcher) -> L3 (Scene):** Requests spatial lookup.
- **L1 (Dispatcher) -> L5 (Component):** Triggers logic hooks.
