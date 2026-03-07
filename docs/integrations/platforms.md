# Platform Support (Native vs Web) 💻🌐

Rupa Framework is built to be truly cross-platform. While the core logic is platform-agnostic, the Engine and Mobile composites provide specific implementations for Native Desktop, WebAssembly (Wasm), and Mobile.

---

## 🖥️ Native Target (Desktop)

On Windows, Linux, and MacOS, Rupa Framework compiles directly to machine code.
- **Backend:** WGPU uses Vulkan, DX12, or Metal.
- **Windowing:** Native OS windows managed by Winit.
- **Performance:** Direct access to GPU memory and multi-threading.

---

## 🌐 Web Target (Wasm)

Rupa Framework can run in any modern browser via WebAssembly and WebGPU (or WebGL2 fallback).
- **Surface:** The framework binds to an HTML5 `<canvas>` element.
- **Loop:** Instead of a blocking event loop, it uses `requestAnimationFrame` to sync with the browser's refresh rate.

---

## ⌨️ Terminal Target (TUI)

Rupa Framework supports running directly in terminal emulators.
- **Surface:** A grid-based cell buffer.
- **Input:** Standard `stdin` stream with ANSI escape sequence decoding.
- **Portability:** Works over SSH and in minimal CLI environments (e.g., Linux VT, headless servers).
- **Integration:** Rupa Framework can coexist with other JavaScript libraries and CSS on the same page.

---

## 📱 Mobile Target (Android / iOS)

Rupa Framework supports native mobile applications through the `rupa-mobile` composite crate.
- **Backend:** WGPU uses Vulkan (Android) or Metal (iOS).
- **Windowing:** Native OS lifecycle management bridging touch events into the core via Winit.
- **Integration:** The mobile runner manages safe area insets, touch gestures, and the volatile lifecycle of mobile rendering surfaces.

---

## 🛠️ Internal Module Reference
- `Cargo.toml`: Features for `web-sys` and `wasm-bindgen`.
- `crates/rupa-engine/src/platform/dispatcher.rs`: Specific event mapping for web vs native.
