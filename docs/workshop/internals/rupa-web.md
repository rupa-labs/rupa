# `rupa-web` 🌐

**The Browser Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to render Rupa applications in the browser via WebAssembly (WASM) and the DOM API.

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Translates VNode patches into raw JavaScript DOM mutations, bypassing standard Virtual DOM overhead.

## 🛠️ Infrastructure Backends
- **Compilation**: `wasm-bindgen`
- **DOM Bridge**: `web-sys`
- **Execution**: Browser JS Engine

## 🚀 Usage

```rust
use rupa_web::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    RupaApp::new().mount("app-root", MyComponent);
}
```
