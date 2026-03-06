# Module: GUI Window (`gui/window.rs`) 🪟

This module acts as the safe wrapper for the physical OS window. It hides the complexity of Winit's windowing logic behind a clean, procedural interface.

---

## 🧠 Internal Anatomy

### 1. Dependency Inversion
The `Window` struct wraps `Arc<winit::window::Window>`. It ensures that only the necessary methods (size, scale factor, handle) are exposed to the rest of Rupaui, preventing "API leakage."

### 2. DPI Management
Responsible for reporting the window's scale factor. This is used by Layer 1 runners to convert physical pixels (from the OS) to logical pixels (for the framework).

---

## 🗝️ API Anatomy

- `Window::new(...)`: Handshakes with the OS to create a surface.
- `.handle()`: Returns the Arc-pointer to the raw window (required by WGPU).
- `.request_redraw()`: Signals the OS that the UI needs to be repainted.

---

## 🛡️ Reliability
By wrapping the window handle in an `Arc`, Rupaui ensures that the window remains valid across multiple threads (e.g., during asynchronous rendering or event dispatching).
