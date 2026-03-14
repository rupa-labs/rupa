# Web Adapter 🌐

The **Web Adapter** manifests Rupa applications in the browser, leveraging WebAssembly for high-performance frontend logic.

---

## 🏗️ Technical Architecture
- **Runtime**: Compiled to **wasm32-unknown-unknown**.
- **DOM Manifestation**: Maps VNode elements to native HTML elements via `web-sys`.
- **Reactivity Bridge**: Syncs Rupa's thread-safe signals with the browser's single-threaded event loop.

---

## 🎨 Artisan Capabilities
- **Zero-Hydration Loading**: Works with the Server Adapter to provide instant visibility.
- **Native Web Feel**: Full support for CSS transitions, browser history, and standard web interactions.
- **JS Interop**: Easily communicate with existing JavaScript libraries or browser APIs.

---

## 🗝️ Standard Workflow
1.  **Build**: Use the Rupa CLI or `wasm-pack` to compile the masterpiece.
2.  **Mount**: The adapter attaches the Rupa app to a specific DOM element (e.g., `#app`).
3.  **Execution**: The browser's `requestAnimationFrame` drives the Rupa render loop.

[Technical Spec: Web Blueprint](../../workshop/architectures/web-adapter.md)
