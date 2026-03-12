# Crate References 📚

This is the official registry of all crates in the Rupa Framework, organized by the **Atoms and Composites** architectural tiers. Each crate acts as a **Port** or **Adapter** in our Hexagonal Architecture.

---

## 🧱 1. Tier 1: Atoms (The Materials & Tools — Ports & Invariants)
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

## 🛠️ 2. Tier 2: Composites (The Master’s Craft — Kernel & Orchestrator)
Orchestrates Atoms into functional systems.

| Crate | Purpose | Detailed Docs |
| :--- | :--- | :--- |
| **`rupa-core`** | Reconciliation engine & Action Bus. | [View Docs](./crates/rupa-core.md) |
| **`rupa-engine`** | The Agnostic Kernel (App Lifecycle). | [View Docs](./crates/rupa-engine.md) |
| **`rupa-ui`** | High-level semantic UI system. | [View Docs](./crates/rupa-ui.md) |
| **`rupa-md`** | Markdown & MDX engine. | [View Docs](./crates/rupa-md.md) |
| **`rupa-router`** | Reactive navigation system. | [View Docs](./crates/rupa-router.md) |
| **`rupa-tui`** | Terminal UI orchestration. | [View Docs](./crates/rupa-tui.md) |
| **`rupa-macros`** | Procedural macros for component generation and attributes. | [View Docs](./crates/rupa-macros.md) |


---
## 🏪 3. Tier 3: Showrooms (The Finished Showroom — Adapters & Infrastructure)
Platform-specific entry points and implementations.

| Crate | Purpose | Detailed Docs |
| :--- | :--- | :--- |
| **`rupa-desktop`** | GPU rendering via WGPU & Winit. | [View Docs](./crates/rupa-desktop.md) |
| **`rupa-terminal`** | Interactive CLI & TUI facade. | [View Docs](./crates/rupa-terminal.md) |
| **`rupa-web`** | Browser rendering via WASM. | [View Docs](./crates/rupa-web.md) |
| **`rupa-server`** | Backend showroom for SSR and SSG. | [View Docs](./crates/rupa-server.md) |
| **`rupa-mobile`** | Native mobile showroom for Android and iOS. | [View Docs](./crates/rupa-mobile.md) |
| **`rupa-fullstack`** | Unified showroom for hybrid hydration and full-stack development. | [View Docs](./crates/rupa-fullstack.md) |
| **`rupa-headless`** | Logic-only showroom for background tasks and automated tests. | [View Docs](./crates/rupa-headless.md) |
| **`rupa-hybrid`** | Interoperability showroom between Web and Native environments. | [View Docs](./crates/rupa-hybrid.md) |
| **`rupa`** | The Universal Facade (All-in-one). | [View Docs](./crates/rupa.md) |

---

## 🛠️ 4. Developer Tooling
Tools for scaffolding, documentation, and artisan experience.

| Crate | Purpose | Detailed Docs |
| :--- | :--- | :--- |
| **`rupa-cli`** | Aesthetic command-line tool and project initializer. | [View Docs](./crates/rupa-cli.md) |
| **`rupa-docs`** | Interactive documentation builder and viewer for Rupa. | [View Docs](./crates/rupa-docs.md) |
| **`rupa-test`** | (Tier 1) Testing utilities for TDD. | [View Docs](./crates/rupa-test.md) |

