# `rupa-hybrid` 🌉

**The Interoperability Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to embed Rupa Web applications inside native wrappers (like Tauri or Electron).

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Bridges the gap between the Browser DOM environment and Native OS APIs via inter-process communication (IPC).

## 🛠️ Key Features
- **Native IPC Bridge**: Send and receive messages from the native host (e.g., Tauri's `invoke`).
- **Shared State**: Synchronize reactive state between the native rust backend and the WASM frontend.

## 🚀 Usage

```rust
use rupa_hybrid::prelude::*;

#[wasm_bindgen(start)]
pub fn start_hybrid() {
    let app = RupaApp::new();
    
    // Listen for native events (e.g., from Tauri)
    app.on_native_event("window-resized", |payload| {
        println!("Window resized by native host!");
    });
    
    app.mount("app-root", MyComponent);
}
```
