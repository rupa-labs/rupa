# Crate References 📚

This is the official registry of all crates in the Rupa Framework, organized by their structural tier. Each crate acts as a **Port** or **Adapter** in our Hexagonal Architecture.

---

## 🧱 1. Atomic Materials (The DNA & Ports)
Low-level, platform-agnostic crates that handle single responsibilities.

| Crate | Purpose | Detailed Docs |
| :--- | :--- | :--- |
| **`rupa-signals`** | Fine-grained reactivity engine. | [View Docs](./crates/rupa-signals.md) |
| **`rupa-vnode`** | UI Contract & Style models. | [View Docs](./crates/rupa-vnode.md) |
| **`rupa-base`** | Foundational types (Color, Vec2, Id). | [View Docs](./crates/rupa-base.md) |
| **`rupa-motion`** | Animation & Physics engine. | [View Docs](./crates/rupa-motion.md) |
| **`rupa-context`** | Dependency Injection system. | [View Docs](./crates/rupa-context.md) |
| **`rupa-forms`** | Form objects & Validation rules. | [View Docs](./crates/rupa-forms.md) |
| **`rupa-auth`** | Identity & Session management. | [View Docs](./crates/rupa-auth.md) |
| **`rupa-assets`** | Resource loading & caching. | [View Docs](./crates/rupa-assets.md) |
| **`rupa-i18n`** | Internationalization & Translation. | [View Docs](./crates/rupa-i18n.md) |
| **`rupa-net`** | Async I/O & Network resources. | [View Docs](./crates/rupa-net.md) |
| **`rupa-store`** | Persistent state & storage backends. | [View Docs](./crates/rupa-store.md) |
| **`rupa-broadcast`** | Global reactive event bus. | [View Docs](./crates/rupa-broadcast.md) |
| **`rupa-queue`** | Background task orchestration. | [View Docs](./crates/rupa-queue.md) |
| **`rupa-telemetry`** | Observability (Logs, Metrics, Profiling). | [View Docs](./crates/rupa-telemetry.md) |
| **`rupa-test`** | Testing utilities for TDD. | [View Docs](./crates/rupa-test.md) |
| **`rupa-a11y`** | Accessibility Ports & Semantics. | [View Docs](./crates/rupa-a11y.md) |
| **`rupa-console`** | Low-level terminal infrastructure. | [View Docs](./crates/rupa-console.md) |
| **`rupa-canvas`** | Low-level drawing ports. | [View Docs](./crates/rupa-canvas.md) |

---

## 🛠️ 2. Composite Assemblies (The Core)
Orchestrates Atomic Materials into functional systems.

| Crate | Purpose | Detailed Docs |
| :--- | :--- | :--- |
| **`rupa-core`** | Reconciliation engine & Action Bus. | [View Docs](./crates/rupa-core.md) |
| **`rupa-engine`** | The Agnostic Kernel (App Lifecycle). | [View Docs](./crates/rupa-engine.md) |
| **`rupa-ui`** | High-level semantic UI system. | [View Docs](./crates/rupa-ui.md) |
| **`rupa-md`** | Markdown & MDX engine. | [View Docs](./crates/rupa-md.md) |
| **`rupa-router`** | Reactive navigation system. | [View Docs](./crates/rupa-router.md) |
| **`rupa-tui`** | Terminal UI orchestration. | [View Docs](./crates/rupa-tui.md) |


---

## 🏪 3. Artisan Showrooms (The Adapters)
Platform-specific entry points and implementations.

| Crate | Purpose |
| :--- | :--- |
| **`rupa-desktop`** | GPU rendering via WGPU & Winit. |
| **`rupa-terminal`** | Interactive CLI & TUI facade. |
| **`rupa-web`** | Browser rendering via WASM. |
| **`rupa`** | The Universal Facade (All-in-one). |
