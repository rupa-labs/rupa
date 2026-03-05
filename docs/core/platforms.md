# WebAssembly & Platforms

Rupaui is designed to be a first-class citizen on both **Native Desktop** and **WebAssembly (Wasm)** platforms.

## 🕸 WebAssembly (Wasm)

Rupaui utilizes `winit`'s web platform features and `WGPU`'s WebGL/WebGPU backends to deliver high-performance interfaces in the browser.

### Key Features
- **Canvas Integration**: Attach Rupaui to any HTML `<canvas>` via ID.
- **Efficient Syncing**: Minimizes alocations and JS/Rust bridge overhead.
- **Event Normalization**: Touch and Mouse events are unified for a consistent experience.

---

## 💻 Native Desktop

Rupaui runs natively on Windows, macOS, and Linux with full hardware acceleration.

### High-DPI Support
Rupaui automatically detects and handles DPI scaling, ensuring that your "Artisan" pixels remain sharp on 4K and Retina displays.

## 🛠 Target-Specific Configuration

You can use Rust's `cfg` attributes to handle platform-specific logic:

```rust
#[cfg(target_arch = "wasm32")]
fn init_web() {
    // Setup Wasm panic hooks and logging
}

#[cfg(not(target_arch = "wasm32"))]
fn init_native() {
    // Setup file system access
}
```
