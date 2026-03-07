# Platform Integration: Web Runner (`web/runner.rs`) 🌐

The **Web Runner** is the execution shell for browser-based applications using WebAssembly (WASM). It leverages the **Canvas API** for rendering and integrates with the browser's event loop via `requestAnimationFrame`.

---

## 🏗️ Architecture

The Web Runner adheres to the 9-layer architecture by bridging the gap between browser APIs and Rupa Framework's agnosticism.

### Key Responsibilities
- **Canvas Management**: Attaches Rupa Framework's rendering surface to a specific `<canvas>` element.
- **Async Lifecycle**: Handles the asynchronous nature of WASM and browser environments.
- **Event Mapping**: Translates browser `MouseEvent`, `KeyboardEvent`, and `TouchEvent` into Rupa Framework's `InputEvent`.
- **Responsive Sizing**: Synchronizes the canvas dimensions with the browser window or its parent container.

---

## 🗝️ API & Usage

### Starting a Web App
To run a Rupa Framework application on the web, you must provide the ID of the target canvas:

```rust
App::new("My Web App")
    .run_web("rupa-canvas");
```

### Execution Flow
1. **Bootstrap**: Initializes `WebInfra` and checks for the canvas in the DOM.
2. **Resumed Hook**: Triggered by Winit; creates the `wgpu` surface and initializes the `Renderer`.
3. **Paint Loop**: Responds to `RequestRedraw` by executing the component paint tree and submitting commands to the GPU.

---

## 🛡️ Dependency Inversion (WebInfra)

All direct calls to `web-sys` are encapsulated in `WebInfra` (`src/platform/web/infra.rs`). This ensures that the runner logic remains focused on orchestration, not DOM manipulation details.
