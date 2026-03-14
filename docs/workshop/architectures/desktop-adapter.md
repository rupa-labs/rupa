# Desktop Adapter Architecture 🖥️

The **Desktop Adapter** is a Tier 3 Showroom that manifests Rupa applications as native windowed software with hardware-accelerated graphics.

---

## 1. Technical Stack

- **Graphics Engine**: Powered by **WGPU** for cross-platform GPU access.
- **Windowing**: Managed by **Winit** for OS-level event loops and window state.
- **Text Shaping**: Uses **Glyphon** for high-quality, hardware-accelerated text rendering.

---

## 2. Core Responsibilities

- **Surface Management**: Creating and resizing the GPU swapchain.
- **Event Mapping**: Translating OS mouse and keyboard events into Rupa **Pointer** and **Focus** intents.
- **Physical Rendering**: Implementing the `Renderer Port` to draw primitives using specialized shaders.
