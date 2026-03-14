# Hardware Adapters (Manifestation) 🏪

Hardware Adapters are the **Physical Body** of the Rupa Framework. They provide the platform-specific glue that manifests your agnostic UI logic onto specific hardware environments.

The Rupa ecosystem currently provides **8 Hardware Adapters** to fulfill any artisan vision:

---

## 🏛️ Pure Targets
Run your application natively on a single runtime.

*   **[Desktop Adapter](./desktop.md)**: Hardware-accelerated GPU graphics (WGPU).
*   **[Terminal Adapter](./terminal.md)**: Aesthetic ANSI character-grid (Crossterm).
*   **[Web Adapter](./web.md)**: Browser runtime via WebAssembly (WASM).
*   **[Mobile Adapter](./mobile.md)**: Native iOS and Android (Vulkan/Metal).
*   **[Server Adapter](./server.md)**: HTML serialization and streaming (SSR/SSG).
*   **[Headless Adapter](./headless.md)**: Logic-only virtual execution for CI and Testing.

---

## 🌌 Fusion Targets
Blending multiple runtimes for advanced architectural needs.

*   **[Hybrid Adapter](./hybrid.md)**: Seamless interoperability between Native and Web.
*   **[Fullstack Adapter](./fullstack.md)**: Synchronized Server-Client orchestration.

---

## 🧪 The "Adapter-Less" Logic
Remember that your components are **Agnostic by Default**. You can develop and test your entire UI logic using the **Headless Adapter** before deciding which physical body it should inhabit.
