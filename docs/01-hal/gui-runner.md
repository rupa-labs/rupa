# Module: GUI Runner (`gui/mod.rs`) 🖼️🏃

The GUI Runner is the implementation of the platform shell for graphical environments. it bridges the asynchronous world of Winit with the high-performance drawing of WGPU.

---

## 🧠 Internal Anatomy

### 1. Winit Handler
- **Mechanism:** Implements `ApplicationHandler`. It listens for native OS signals (Resume, Suspend, Resize) and translates them into framework-ready instructions.

### 2. Composition Shell
- **Composition:** It wraps **`PlatformCore`** (L1) and **`GuiRenderer`** (L2).
- **Responsibility:** Coordinates the "Big Loop" (Input -> Layout -> Paint -> Present).

---

## 🗝️ Logic & Flow

### Redraw Handling
When `RedrawRequested` is triggered:
1. It queries `self.core.compute_layout()`.
2. It initializes a WGPU frame via the renderer.
3. It traverses the tree to execute `component.paint()`.
4. It calls `renderer.present()` to flip the GPU buffers.

### Event Dispatching
Translates native Winit mouse/keyboard enums into `InputEvent` before passing them to the `InputDispatcher`.

---

## 🔄 Interaction Flow
- **L1 (HAL) -> L2 (Renderer):** Manages the WGPU device lifecycle.
- **L1 (HAL) -> L1 (PlatformCore):** Coordinates tree updates.
