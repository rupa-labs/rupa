# Platform Integration: Desktop Runner (`desktop/runner.rs`) 🖥️

The **Desktop Runner** is the primary execution shell for macOS, Windows, and Linux applications. It utilizes **Winit** for OS window management and **WGPU** for high-performance, hardware-accelerated rendering.

---

## 🏗️ Architecture

The Desktop Runner adheres to the 9-layer arhcitecture by bridging OS events into the Rupa Framework ecosystem.

### Key Responsibilities
- **Window Lifecycle**: Manages the creation and destruction of the OS window.
- **Event Dispatching**: Maps native `WindowEvent` (Resize, Close, Cursor) into Rupa Framework's `InputEvent`.
- **HiDPI Scaling**: Automatically handles pixel density changes and logical coordinate mapping.
- **Paint Coordination**: Triggers the rendering pipeline based on OS redraw requests.

---

## 🗝️ API & Usage

### Starting a Desktop App
Desktop apps are typically started via the standard `App::run()` method, which uses the `desktop` feature:

```rust
App::new("Artisan App")
    .run();
```

### Execution Flow
1. **Bootstrap**: Initializes the Winit event loop.
2. **Resumed**: Creates the OS window via `DesktopInfra` and initializes the `Renderer`.
3. **Event Loop**: Responds to user input by dispatching events through the `InputDispatcher`.
4. **Redraw**: Executes the component paint tree and submits commands to the GPU.

---

## 🛡️ Dependency Inversion (DesktopInfra)

All direct calls to `winit`'s window creation are encapsulated in `DesktopInfra` (`crates/rupa-engine/src/platform/desktop/infra.rs`). This ensures that the runner logic remains focused on orchestration, not OS-specific windowing calls.
