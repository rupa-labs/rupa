# Architectural References Index 🧭

This document provides a comprehensive registry of all architectural systems within the Rupa Framework. It serves as a map for **Workshop Contributors** to understand the relationship between the **Artisan Brand Identity** (the story) and the **Technical Unit Names** (the standard).

---

## 🧱 Tier 1: Atoms (Materials)
**Technical Context**: Ports & Invariants.
The foundational, platform-agnostic materials and port definitions.

| System Unit | Crate | Description |
| :--- | :--- | :--- |
| **Reactivity Port** | `rupa-signals` | Fine-grained heartbeat (Signals, Memos, Effects). |
| **UI Contract Port** | `rupa-vnode` | Agnostic UI Language (VNodes and Styles). |
| **Renderer Port** | `rupa-core` | The foundational contract for all drawing backends. |
| **Base Port** | `rupa-base` | Invariants: `Vec2`, `Color`, `Id`, and Error types. |
| **Motion Port** | `rupa-motion` | Physics-based animation and spring transitions. |
| **Identity Port** | `rupa-auth` | Interface for authentication and RBAC protocols. |
| **Broadcast Port** | `rupa-broadcast`| Global event bus and channel-based communication. |
| **Storage Port** | `rupa-store` | Interface for reactive persistence. |
| **Network Port** | `rupa-net` | Interface for agnostic async networking. |
| **I18n Port** | `rupa-i18n` | Interface for localization and translation. |
| **Context Orchestrator** | `rupa-context` | Dependency Injection and Scoped Registry system. |
| **Asset Port** | `rupa-assets` | Resource loading, caching, and management. |
| **Queue Port** | `rupa-queue` | Background task orchestration and async jobs. |
| **Console Port** | `rupa-console` | Low-level terminal infrastructure and CLI primitives. |
| **Telemetry Port** | `rupa-telemetry` | Observability hub: Logging, Metrics, and Profiling. |
| **Canvas Port** | `rupa-canvas` | Low-level hardware-agnostic drawing API. |
| **A11y Port** | `rupa-a11y` | Semantic accessibility and screen reader integration. |
| **Testing Orchestrator** | `rupa-test` | TDD tools for visual and structural verification. |

---

## 🛠️ Tier 2: Composites (The Master's Craft)
**Technical Context**: Kernel & Orchestrator.
The agnostic "Brain" that orchestrates Atoms into functional logic.

| System Unit | Module / Crate | Description |
| :--- | :--- | :--- |
| **Reconciliation Kernel** | `rupa-core` | High-performance VNode diffing and patching engine. |
| **Input Dispatcher** | `rupa-core` | Abstracting hardware into semantic "Intents". |
| **Layout Kernel** | `rupa-core` | Pixel-to-Cell translation and layout solving. |
| **Action Bus** | `rupa-core` | Asynchronous intent-based messaging bridge. |
| **Component Kernel** | `rupa-core` | Semantic lifecycle and hooks management. |
| **Scene Manager** | `rupa-core` | Spatial tracking and geometric hit-testing. |
| **Plugin Orchestrator** | `rupa-engine` | The gateway for modular framework extensibility. |
| **Element Tree** | `rupa-engine` | Stateful management of the component hierarchy. |
| **App Lifecycle** | `rupa-engine` | The agnostic runner and platform bridge (`App`). |
| **Agnostic UI** | `rupa-ui` | The standard semantic component library. |
| **Markdown Engine** | `rupa-md` | Content-driven VNode generation from MDX. |
| **Router Kernel** | `rupa-router` | Reactive hierarchical navigation and URL mapping. |
| **Forms Orchestrator** | `rupa-forms` | Reactive form state and schema-driven validation. |
| **TUI Orchestrator** | `rupa-tui` | High-level terminal UI layout and interaction. |
| **Artisan Macros** | `rupa-macros` | Procedural macros for ergonomic code generation. |

---

## 🏪 Tier 3: Showrooms (The Finished Showroom)
**Technical Context**: Adapters & Infrastructure.
Target-specific adapters that manifest the Kernel onto physical hardware.

| System Unit | Crate | Target Hardware / API |
| :--- | :--- | :--- |
| **Desktop Adapter** | `rupa-desktop` | WGPU & Winit Graphics Adapter. |
| **Terminal Adapter** | `rupa-terminal` | Crossterm Character-grid Adapter. |
| **Web Adapter** | `rupa-web` | WASM, DOM & WebGPU Adapter. |
| **Mobile Adapter** | `rupa-mobile` | Android/iOS Native Adapter. |
| **Server Adapter** | `rupa-server` | SSR & API Backend Adapter. |
| **Headless Adapter** | `rupa-headless` | CI & Automation Adapter. |
| **Fullstack Adapter** | `rupa-fullstack` | Hydration and Universal Interop. |
| **Hybrid Adapter** | `rupa-hybrid` | Web-in-Native embedding (Interoperability). |
| **Universal Facade** | `rupa` | Unified high-level entry point for Artisans. |

---

## 🧩 Shared Architectures
Cross-cutting systems used by both Artisans and Contributors.

*   **Theme Orchestrator**: Global aesthetic presets and OKLCH color scaling.
*   **Standard Layouts**: Universal Flexbox/Grid patterns (Taffy based).
*   **CLI Orchestrator**: Scaffolding and developer experience utility (`rupa-cli`).
*   **Doc Orchestrator**: Interactive documentation builder and viewer (`rupa-docs`).

---

*This index is a living document. Every new crate or architectural sub-system added to the Rupa Framework MUST be registered here to maintain the **3S Doctrine** (Scalable, Sustain, Secure).*
