# Module: GUI Renderer (`crates/rupa-engine/src/renderer/gui/renderer.rs`) 🎨

The GUI Renderer is the orchestration unit for hardware-accelerated drawing. It manages the GPU pipeline and coordinates the flow of geometric and typographic data.

---

## 🧠 Internal Anatomy

### 1. Resource Management
Maintains the active handles for the **WGPU Device**, **Queue**, and **Surface**. It is responsible for the frame lifecycle: `begin_frame` (acquiring a swapchain texture) and `present` (submitting command buffers).

### 2. Composition Shell
- **Internal References:** 
    - `crates/rupa-engine/src/renderer/gui/batcher.rs`
    - `crates/rupa-engine/src/renderer/gui/text_renderer.rs`
- **Responsibility:** It acts as the traffic controller, sending rectangle requests to the Batcher and string data to the Text Engine.

---

## 🗝️ API Anatomy

- **`new(window)`**: Performs the async handshake with the physical GPU and initializes the WGPU surface.
- **`render_patch(patch)`**: The primary entry point. Iterates through structural UI changes and updates the **Batcher** and **Text Renderer** accordingly.
- **`present()`**: Submits all accumulated draw calls to the GPU queue and presents the frame.

---

## 🔄 Interaction Flow

1.  **Reconciliation**: Receives `Patch` instructions from the VNode core.
2.  **Geometry Processing**: Elements (Div, Flex, etc.) are converted into vertex data via the **Batcher**.
3.  **Typography Processing**: Text nodes are sent to the **Text Renderer** for shaping and layout.
4.  **Hardware Submission**: The `present()` call finalizes the frame.
