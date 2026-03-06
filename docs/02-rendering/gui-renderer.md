# Module: GUI Renderer (`gui/renderer.rs`) 🎨

The GUI Renderer is the orchestration unit for hardware-accelerated drawing. It manages the GPU pipeline and coordinates the flow of geometric and typographic data.

---

## 🧠 Internal Anatomy

### 1. Resource Management
Maintains the active handles for the **WGPU Device**, **Queue**, and **Surface**. It is responsible for the frame lifecycle: `begin_frame` (acquiring a swapchain texture) and `present` (submitting command buffers).

### 2. Composition Shell
- **Composition:** Wraps `RenderCore` (L2), `Batcher` (L2), `TextRenderer` (L2), and `StagingBelt` (WGPU).
- **Responsibility:** It acts as the traffic controller, sending rectangle requests to the Batcher and string data to the Text Engine.

---

## 🗝️ API Anatomy

- `new(window)`: Performs the async handshake with the physical GPU.
- `resize()`: Synchronizes the swapchain with new window dimensions.
- `draw_rect()`: Transforms coordinates using the camera state before buffering vertex data.

---

## 🔄 Interaction Flow
- **L2 (GUI Renderer) -> L2 (Batcher):** Feeds calculated vertex data.
- **L2 (GUI Renderer) -> L2 (Text Engine):** Supplies shaped text buffers for rendering.
