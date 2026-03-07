# Rupa Framework Architecture 🏛️

This document outlines the **Atoms & Composites** architecture of the Rupa Framework. It is designed with a high-performance modular philosophy, allowing developers to either use pre-assembled solutions or build their own custom framework from individual atomic pieces.

---

## 🏗️ The Layered Blueprint: Atoms to Facade

Rupa Framework is organized into three distinct tiers within a monorepo workspace.

### 1. The Atoms (Low-Level Units - The Materials)
These are independent, low-level crates that handle a single responsibility. They are the "screws and wood" of the framework.
*   **`rupa-signals`**: The fine-grained reactivity engine (Signal, Memo, Effect).
*   **`rupa-styling`**: The visual DNA, OKLCH color math, and unified design tokens.
*   **`rupa-vnode`**: The agnostic Virtual Tree structure used as a universal interface.
*   **`rupa-id`**: Secure unique identifier generation.
*   **`rupa-layout-taffy`**: High-performance geometric resolution (Taffy integration).

### 2. The Composites (High-Level Assemblies - The Furniture)
These crates assemble multiple atoms into functional, high-level modules. They are pre-built solutions for standard use cases.
*   **`rupa-ui`**: The Artisan Component Library (`Button`, `Text`, `VStack`, etc.).
*   **`rupa-engine`**: The Native Runtime. Handles hardware-accelerated rendering for **Desktop and Mobile (GPU)** and Terminal (TUI).
*   **`rupa-server`**: The Backend & SSR Engine. Handles HTML generation and Axum integration.
*   **`rupa-client`**: The Web Frontend Engine. Handles DOM manipulation and WASM Hydration.
*   **`rupa-mobile`**: (Strategic) The Mobile Integration bridge. Handles native platform interop (JNI for Android, C-FFI for iOS) and touch-optimized event dispatching.
*   **`rupa-macros`**: Procedural stencils for reducing boilerplate across all layers.

### 3. The Facade (The Showroom)
*   **`rupa`**: The primary entry point for users. It orchestrates all composites via **Feature Flags**, allowing for a tailored development experience.

---

## 📱 The Mobile Strategy (Touch & Gesture)

Mobile platforms require specialized handling beyond standard GPU rendering:
-   **Touch-First Interaction:** `rupa-core` and `rupa-engine` are designed to support complex gestures (Long Press, Swipe, Pinch) via a specialized touch-mapping layer.
-   **Native Shells:** Mobile apps run within a native shell (Activity on Android, UIViewController on iOS) while the UI is rendered via WGPU.
-   **Platform Bridge:** `rupa-mobile` provides access to native mobile APIs (Camera, GPS, Notifications) through a unified Rust interface.

---

## 🛠️ The Modular Choice

Because of our **Zero-Cost Abstraction** and modular design, you can mix and match atoms to create a custom runtime.

| Target Application | Required Components (Atoms & Composites) |
| :--- | :--- |
| **Native Desktop** | `rupa-ui` + `rupa-engine` (WGPU) |
| **Native Mobile** | `rupa-ui` + `rupa-engine` (WGPU) + `rupa-mobile` |
| **Native Terminal** | `rupa-ui` + `rupa-engine` (TUI) |
| **Full-Stack Web** | `rupa-ui` + `rupa-server` + `rupa-client` |
| **Embedded/Headless**| `rupa-signals` + `rupa-vnode` |
| **JS/TS Hybrid** | `rupa-core` + `rupa-client` (WASM) |

---

## 🔄 Architectural Principles

1.  **Atomic Independence:** Atoms must never depend on Composites.
2.  **Headless-First:** Every Atom and UI component must be testable without a display or OS environment.
3.  **Industry-Standard Naming:** We use clear, industry-standard names for crates while maintaining an artisan spirit.
4.  **Serialization DNA:** All data moving between atoms and composites is serializable via **Serde**.
