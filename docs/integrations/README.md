# Platform Integrations 🔌

Detailed specifications for how the Rupa Core connects to external systems and hardware APIs.

---

## 🏃 Platform Run-loops
*   **[Desktop Runner](./desktop-runner.md)**: Winit-based event loop orchestration.
*   **[Web Runner](./web-runner.md)**: JS-bridged main loop for WASM.
*   **[Terminal Runner](./terminal-runner.md)**: Crossterm event processing.
*   **[Mobile Runner](./mobile-runner.md)**: Native mobile lifecycle management.

---

## ⌨️ Input & Interaction
*   **[Input Events](./input-events.md)**: Mapping raw OS signals to UIEvent.
*   **[Input Dispatcher](./input-dispatcher.md)**: The hit-testing and event routing engine.

---

## 🎨 Hardware & System APIs
*   **[Graphics API](./graphics-api.md)**: WGPU abstraction layer.
*   **[Desktop Window](./desktop-window.md)**: Low-level window management.
*   **[Platform Orchestrator](./platform-orchestrator.md)**: Managing multiple concurrent adapters.
*   **[Platforms Index](./platforms.md)**: A registry of all supported integration targets.

---

## 🛠️ Infrastructure Adapters
*   **[A11y Bridge](./a11y-bridge.md)**: Accessibility tree synchronization.
*   **[Asset Management](./asset-management.md)**: Pipeline for platform-specific asset loading.
*   **[Network I/O](./network-io.md)**: Native and Web network client adapters.
*   **[Backend Selection](./backend-selection.md)**: Logic for choosing the best store backend.
*   **[Telemetry](./telemetry.md)**: Structured log and metrics capture.
