# Reactivity: Change Propagation 🔄

Rupa Framework follows a **Reactive Pipeline** that ensures data changes flow smoothly from the logic layer down to the hardware. This document tracks the journey of a single state change.

---

## 🔁 The Lifecycle of a Change

### 1. Trigger (Layer 5/7)
A component's logic mutates a signal, for example: `counter.update(|v| *v += 1)`.

### 2. Notification (Layer 4)
The `Signal` notifies its internal observers. In Rupa Framework, the default observer is the **Platform Redraw Hook**.

### 3. Redraw Request (Layer 1)
`rupa::platform::request_redraw()` is called. 
- **In GUI:** This sends a `UserEvent` to Winit's event loop.
- **In TUI:** This breaks the polling sleep to trigger a new frame.

### 4. Scene Resolution (Layer 3)
The Platform Runner calls `SceneCore::resolve()`. Because the state has changed, components marked as `dirty` recalculate their Taffy nodes.

### 5. Visual Output (Layer 2)
The Renderer traverses the newly resolved tree and flushes draw commands to the GPU or Terminal.

---

## 🛡️ Reactive Integrity

- **Predictability:** Changes always flow in one direction (Signal -> Redraw -> Layout -> Paint).
- **Efficiency:** The framework does not "poll" for changes. If no signal changes, the GPU stays idle (0% CPU usage).
- **Atomicity:** Multiple signal changes within the same event handler will only trigger a single redraw request, preventing "frame stutter."
