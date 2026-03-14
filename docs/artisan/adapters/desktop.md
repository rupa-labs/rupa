# Desktop Adapter 🖥️

The **Desktop Adapter** is the high-performance showroom for Rupa Framework native applications. It bridges the Rupa Kernel with the operating system's graphics API and windowing system.

---

## 🏗️ Technical Architecture
- **Rendering Engine**: Uses **WGPU** to provide a cross-platform abstraction over Vulkan, Metal, and DirectX.
- **Event Loop**: Powered by **Winit**, ensuring native performance and responsive window handling.
- **Typography**: High-quality text rendering via **Glyphon**, supporting sub-pixel anti-aliasing and complex shaping.

---

## 🎨 Artisan Capabilities
- **Direct GPU Control**: Inject custom WGSL shaders into your components via the `Canvas Port`.
- **Native Performance**: Zero-overhead communication between reactive logic and drawing commands.
- **Multitasking**: Full support for OS-level windows, menus, and system tray interactions.

---

## 🗝️ Standard Workflow
1.  **Initialization**: `App::new("My App")` automatically targets the Desktop Adapter if the `desktop` feature is enabled.
2.  **Manifestation**: Every frame, the Desktop Adapter consumes VNode patches and updates GPU vertex buffers.
3.  **Interaction**: Native mouse and keyboard events are translated into Rupa **Pointer** and **Focus** intents.

[Technical Spec: Desktop Blueprint](../../workshop/architectures/desktop-adapter.md)
