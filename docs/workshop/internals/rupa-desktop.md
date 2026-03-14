# `rupa-desktop` 🖥️

**The Native GPU Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to render Rupa applications natively using WGPU (Hardware Acceleration) and Winit (OS Windowing).

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Translates platform-agnostic `Patch` instructions from the Core Reconciler into optimized GPU draw calls.

## 🛠️ Infrastructure Backends
- **Graphics**: `wgpu`
- **Windowing**: `winit`
- **Text Rendering**: `glyphon`
- **Layout**: `taffy`

## 🚀 Usage

```rust
use rupa_desktop::prelude::*;

fn main() {
    DesktopRunner::new(App::new("My Desktop App").root(MyComponent))
        .run()
        .unwrap();
}
```
