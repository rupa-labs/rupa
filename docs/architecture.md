# Rupa Framework Architecture 🏛️

This document outlines the **Atomic Materials, Agnostic Kernel, & Artisan Showrooms** architecture of the Rupa Framework. It is designed with a high-performance modular philosophy, allowing developers to either use pre-assembled solutions or build their own custom framework from individual atomic pieces.

---

## 🏗️ The Layered Blueprint: DNA to Showroom

Rupa Framework follows a "Take what you want" philosophy, organized into three distinct tiers:

### 1. Tier 1: Atomic Materials (The DNA) 🧱
These are independent, low-level crates that handle a single responsibility. They are the agnostic building blocks of the framework.
*   **`rupa-signals`**: The fine-grained reactivity engine.
*   **`rupa-vnode`**: The universal UI representation and Style models.
*   **`rupa-motion`**: Animation engine and spring physics.
*   **`rupa-md`**: Markdown & MDX parsing engine.
*   **`rupa-support`**: Math, ID generation, and common types.

### 2. Tier 2: Composite Assemblies (The Kernel) 🛠️
These crates orchestrate materials into functional systems without being tied to specific hardware.
*   **`rupa-core`**: The reconciliation engine (Diff/Patch), event definitions, and **Artisan Action Bus**.
*   **`rupa-engine`**: The **Agnostic Kernel**. Manages the global `App` lifecycle, plugin orchestration, and action dispatching.
*   **`rupa-ui`**: The high-level **UI System**.

### 3. Tier 3: Artisan Showrooms (The Implementation) 🏪
Platform-specific shells that provide the "physical body" for your application.
*   **`rupa-desktop`**: High-performance GPU rendering via **WGPU & Winit**.
*   **`rupa-tui`**: Aesthetic terminal rendering via **Crossterm**.
*   **`rupa-web`**: Browser-based rendering via **WASM & DOM**.
*   **`rupa-mobile`**: Targeted at native mobile experiences.
*   **`rupa-headless`**: Logic-only entry point for automation and testing.

---

## 🎨 Philosophy: Take What You Want

By decoupling the **Kernel** (`rupa-engine`) from the **Showroom**, Rupa ensures maximum efficiency:

1.  **Lightweight CLI**: Tools like `rupa-cli` only pull the Kernel and TUI materials (~50 dependencies total).
2.  **Zero-Overhead Headless**: Background services can use the full power of Rupa's reactivity without any rendering libraries.
3.  **Opt-in Heavyweight**: Large dependencies like WGPU are only compiled when building a Desktop application.

---

## 🔄 Architectural Principles

1.  **Strict Agnosticism**: The Kernel (`rupa-engine`) and Core must never import platform-specific crates like `wgpu` or `crossterm`.
2.  **Platform Runners**: Each Showroom provides a `PlatformRunner` that implements how the Agnostic Kernel's commands are executed on that hardware.
3.  **Headless-First**: Every UI component must be renderable to a VNode and testable without an OS environment.
4.  **Serialization DNA**: All data moving between layers must be serializable via **Serde** to support the Polyglot vision.
