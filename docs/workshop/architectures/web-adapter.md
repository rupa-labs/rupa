# Web Adapter Architecture 🌐

The **Web Adapter** is a Tier 3 Showroom that targets browser environments using WebAssembly (WASM).

---

## 1. Execution Modes

### A. Native DOM Adapter
Maps VNodes directly to HTML elements for optimal accessibility and SEO.
- **Backend**: Uses `web-sys` and `js-sys` for direct DOM manipulation.

### B. Canvas/WebGPU Adapter
Renders the high-performance graphics engine inside an HTML5 Canvas or WebGPU surface.

---

## 2. Core Responsibilities

- **Hydration**: Re-attaching reactive state to pre-rendered HTML (from SSR).
- **Web Interaction**: Mapping browser events (Click, Touch, Keydown) to Rupa intents.
- **JS Interop**: Facilitating data exchange between Rust logic and the JavaScript ecosystem.
