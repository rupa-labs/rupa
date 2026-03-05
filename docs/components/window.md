# Window Component

The `Window` is the top-level container of a **Rupaui** application. It manages the physical viewport provided by the Operating System or the browser's `<canvas>` element.

## 📐 Usage

### Native Desktop
```rust
let window = Window::new(
    &event_loop, 
    "Artisan's Atelier", 
    1024, 
    768
).expect("Failed to create window");
```

### WebAssembly (Wasm)
On the web, you can optionally provide a `canvas_id` to attach Rupaui to an existing HTML element.

```rust
let window = Window::new(
    &event_loop, 
    "Rupa Web", 
    800, 
    600,
    Some("main-canvas") // Optional canvas ID
).expect("Failed to create web window");
```

## 🛠 Responsibilities
- **Hardware Abstraction**: Wraps `winit::window::Window`.
- **GPU Surface**: Acts as the primary target for `WGPU` rendering.
- **DPI Scaling**: Automatically handles high-DPI displays (Retina/4K).
