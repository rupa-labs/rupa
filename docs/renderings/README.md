# Rendering Systems 🎨

Technical specifications for the various hardware-accelerated and text-based rendering pipelines.

---

## ⚙️ Universal Interface
*   **[Renderer Interface](./renderer-interface.md)**: The Port trait for all rendering implementations.

---

## 💻 GUI Rendering (WGPU)
*   **[GUI Renderer](./gui-renderer.md)**: High-level GPU rendering orchestrator.
*   **[GUI Backend](./gui-backend.md)**: Managing WGPU device and surface state.
*   **[GUI Batcher](./gui-batcher.md)**: Optimizing draw calls for performance.
*   **[GUI Text Renderer](./gui-text-renderer.md)**: GPU-accelerated typography.
*   **[GUI Texture](./gui-texture.md)**: Asset and image buffer management.

---

## ⌨️ TUI Rendering (Terminal)
*   **[TUI Renderer](./tui-renderer.md)**: Grid-based text cell rendering.

---

## 📐 Specialized Engines
*   **[Typography Engine](./typography-engine.md)**: Low-level font and glyph management.
*   **[Vector Engine](./vector-engine.md)**: Path and shape tessellation for GPU.
*   **[Canvas](./canvas.md)**: High-level drawing API integration.
