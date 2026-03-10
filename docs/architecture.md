# Rupa Framework Architecture 🏛️

This document outlines the **Atomic Materials & Composite Assemblies** architecture of the Rupa Framework. It is designed with a high-performance modular philosophy, allowing developers to either use pre-assembled solutions or build their own custom framework from individual atomic pieces.

---

## 🏗️ The Layered Blueprint: Atomic Materials to Artisan Showrooms

Rupa Framework is organized into three distinct tiers within a monorepo workspace. For a deeper understanding of this design pattern, see the **[Artisan Workshop Standard Deep Dive](./architectures/workshop-tiers.md)**.

### 1. Tier 1: Atomic Materials 🧱
These are independent, low-level crates that handle a single responsibility. They are the agnostic building blocks of the framework.
*   **`rupa-support`**: Math (Vec2), ID generation, and common Errors.
*   **`rupa-signals`**: The fine-grained reactivity engine.
*   **`rupa-vnode`**: The "Universal Language & DNA" (Virtual Tree + Style Data).
*   **`rupa-store`**: Persistent storage abstraction (SQLite, FileSystem).
*   **`rupa-net`**: Reactive network I/O and async data fetching.
*   **`rupa-motion`**: Animation engine and spring physics.
*   **`rupa-router`**: Agnostic navigation and view transitions.
*   **`rupa-i18n`**: Multi-language dictionary and formatting.
*   **`rupa-assets`**: Resource loading and caching pipeline.
*   **`rupa-a11y`**: OS accessibility bridge.
*   **`rupa-auth`**: Identity, RBAC, and Team management.
*   **`rupa-forms`**: Reactive validation and form state tracking.
*   **`rupa-context`**: Tree-scoped dependency injection.
*   **`rupa-canvas`**: Low-level hardware-accelerated drawing.

### 2. Tier 2: Composite Assemblies 🛠️
These crates assemble multiple atomic materials into functional, high-level systems.
*   **`rupa-core`**: The primary foundation. Integrates VNodes and handles diffing/patching reconciliation.
*   **`rupa-ui`**: The **UI System**. It houses the **UI Component System** (Semantic elements) and the **UI Utilities System** (Styling API).
*   **`rupa-engine`**: The Native Runtime. Handles hardware-accelerated rendering for **Desktop and Mobile (GPU)** and Terminal (TUI).
*   **`rupa-server-core`**: The Backend & SSR Engine. Handles HTML generation and Axum integration.
*   **`rupa-web-core`**: The Web Frontend Engine. Handles DOM manipulation and WASM Hydration.
*   **`rupa-mobile-core`**: The Mobile Integration bridge. Handles native platform interop (JNI for Android, C-FFI for iOS).
*   **`rupa-macros`**: Procedural stencils for reducing boilerplate across all layers.

### 3. Tier 3: Artisan Showrooms 🏪
Specialized entry points pre-assembled for specific business use cases.
*   **`rupa`**: The universal showroom (All features).
*   **`rupa-desktop`**: Pre-assembled for Native GUI apps.
*   **`rupa-web`**: Optimized for WASM and modern frontend development.
*   **`rupa-server`**: Built for SSR and Backend services.
*   **`rupa-tui`**: Specialized for interactive terminal tools (TUI).
*   **`rupa-mobile`**: Targeted at native mobile experiences.
*   **`rupa-headless`**: Logic-only entry point for automation.
*   **`rupa-hybrid`**: Bridge facade for Rust-JS/TS interop.

---

## 📱 The Mobile Strategy (Touch & Gesture)

Mobile platforms require specialized handling beyond standard GPU rendering:
-   **Touch-First Interaction:** `rupa-core` and `rupa-engine` are designed to support complex gestures (Long Press, Swipe, Pinch) via a specialized touch-mapping layer.
-   **Native Shells:** Mobile apps run within a native shell (Activity on Android, UIViewController on iOS) while the UI is rendered via WGPU.
-   **Platform Bridge:** `rupa-mobile-core` provides access to native mobile APIs (Camera, GPS, Notifications) through a unified Rust interface.

---

## 🛠️ The Modular Choice

Because of our **Zero-Cost Abstraction** and modular design, you can mix and match atomic materials to create a custom runtime.

| Target Application | Required Components (Atomic Materials & Composite Assemblies) |
| :--- | :--- |
| **Native Desktop** | `rupa-ui` + `rupa-engine` (WGPU) |
| **Native Mobile** | `rupa-ui` + `rupa-engine` (WGPU) + `rupa-mobile-core` |
| **Native Terminal** | `rupa-ui` + `rupa-engine` (TUI) |
| **Full-Stack Web** | `rupa-ui` + `rupa-server-core` + `rupa-web-core` |
| **Frontend (WASM)** | `rupa-ui` + `rupa-web-core` |
| **Backend (SSR/API)** | `rupa-server-core` + `rupa-core` |
| **Embedded/Headless**| `rupa-signals` + `rupa-vnode` |
| **JS/TS Hybrid** | `rupa-core` + `rupa-web-core` (WASM) |

---

## 🔄 Architectural Principles

1.  **Atomic Independence:** Atomic Materials must never depend on Composite Assemblies.
2.  **Headless-First:** Every Atomic Material and UI component must be testable without a display or OS environment.
3.  **Industry-Standard Naming:** We use clear, industry-standard names for crates while maintaining an artisan spirit.
4.  **Serialization DNA:** All data moving between atomic materials and composite assemblies is serializable via **Serde**.
