# Module: GUI Runner (`src/platform/gui/mod.rs`) 🖥️

The GUI Runner is the hardware-accelerated backend for Rupaui, built on top of **Winit** (Windowing) and **WGPU** (Graphics).

---

## 🏗️ Technical Architecture

### 1. Robust Lifecycle
The GUI Runner implements the `ApplicationHandler` pattern from Winit, ensuring reliable startup and resume behavior across Windows, MacOS, and Linux.
- **Error Handling:** All window creation and event loop builds are `Result`-based, avoiding abrupt crashes.
- **Resource Management:** Automatically initializes and manages the **WGPU Renderer** context.

### 2. Precise Coordination (HiDPI)
The Runner handles the conversion between **Physical Pixels** (OS units) and **Logical Pixels** (Framework units).
- **Scale Factor:** Automatically applied to mouse coordinates, scroll deltas, and window sizes.
- **Layout Consistency:** Ensures UI elements look identical across different display densities.

### 3. Isolated Input Mapping (`input.rs`)
To prevent "God File" issues, key mapping logic is isolated.
- **Logical Mapping:** Uses `logical_key` and `text` properties from Winit to ensure correct text input regardless of the physical keyboard layout.
- **IME Support:** Native handling of `WindowEvent::Ime` for international character input.

---

## 🗝️ Implementation Details

### `struct GuiRunner`
- `core`: A `SharedPlatformCore` (Arc<RwLock>) for thread-safe access.
- `window`: A `Window` wrapper isolating the renderer from Winit specifics.
- `renderer`: The Layer 2 WGPU backend.
- `modifiers`: Current state of Shift, Ctrl, Alt, and Logo keys.

---

## 🌟 Advanced Display Capabilities

### High-DPI Synchronization (Retina/4K)
Rupaui automatically synchronizes the OS `scale_factor` across all internal layers. This ensures that:
- Coordinates are calculated in logical units for consistent layout.
- Rendering is performed in physical units for pixel-perfect sharpness.
- Element sizes remain consistent regardless of the monitor's pixel density.

### Multi-Window Architecture
The framework is designed to support complex desktop environments where an application can orchestrate multiple independent windows sharing a unified state.

### OS Services Integration
Future updates to the GUI Runner include native bridging for:
- **System Clipboard:** Seamless copy-paste of text and assets.
- **Drag-and-Drop:** Native file and data ingestion into UI components.
- **Shell Customization:** Controls for window transparency, custom titlebars, and fullscreen states.


