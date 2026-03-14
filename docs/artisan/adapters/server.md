# Server Adapter 🛡️

The **Server Adapter** is the non-visual engine of the Rupa ecosystem, responsible for turning components into strings.

---

## 🏗️ Technical Architecture
- **HTML Serialization**: A specialized implementation of the **Renderer Port** that writes to string buffers.
- **Streaming Support**: Can stream large UI trees chunk-by-chunk for faster Time-to-First-Byte (TTFB).
- **Agnostic Logic**: Runs in pure Rust environments without needing a window or GPU.

---

## 🎨 Artisan Focus
- **Perfect SEO**: Search engines receive fully rendered HTML, ensuring your masterpiece is discoverable.
- **Speed**: Instant initial load times by delivering pre-rendered content.
- **Isomorphic Code**: Write your UI once and run it on both server and client.

---

## 🗝️ Standard Workflow
1.  **Rendering**: The server calls `render_to_string()` on an Artisan component.
2.  **Serialization**: The adapter converts the VNode tree into optimized HTML.
3.  **Delivery**: The resulting string is sent to the user's browser via HTTP.

[Technical Spec: Server Blueprint](../../workshop/architectures/server-adapter.md)
