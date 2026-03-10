# Rupa Framework Architecture 🏛️

This document outlines the **Atomic Materials, Agnostic Kernel, & Artisan Showrooms** architecture of the Rupa Framework.

---

## 🏗️ The Layered Blueprint: DNA to Showroom

Rupa Framework follows a "Take what you want" philosophy, organized into three distinct tiers:

### 1. Tier 1: Atomic Materials (The DNA) 🧱
Independent, low-level crates that handle a single responsibility.
*   **`rupa-signals`**: The fine-grained reactivity engine.
*   **`rupa-vnode`**: The universal UI representation and Style models.
*   **`rupa-motion`**: Animation engine and spring physics.
*   **`rupa-console`**: Low-level terminal infrastructure (ANSI, Runner, Raw Mode).
*   **`rupa-base`**: Math, ID generation, and common types.

### 2. Tier 2: Composite Assemblies (The Kernel) 🛠️
Crates that orchestrate materials into functional systems without being tied to specific hardware.
*   **`rupa-core`**: The reconciliation engine (Diff/Patch) and Artisan Action Bus.
*   **`rupa-engine`**: The Agnostic Kernel. Manages the global `App` lifecycle and action dispatching.
*   **`rupa-ui`**: High-level semantic UI system (agnostic).
*   **`rupa-tui`**: Terminal-specific UI components (TUI-optimized Tables, Lists, etc.).
*   **`rupa-md`**: Markdown & MDX parsing engine.

### 3. Tier 3: Artisan Showrooms (The Implementation) 🏪
Platform-specific entry points that provide the "physical body" for your application.
*   **`rupa-desktop`**: High-performance GPU rendering via WGPU & Winit.
*   **`rupa-terminal`**: The flagship Terminal experience (Facade for CLI Apps).
*   **`rupa-web`**: Browser-based rendering via WASM & DOM.
*   **`rupa-headless`**: Logic-only entry point for automation and testing.
*   **`rupa-cli`**: Developer Tooling (Scaffolding, Build).

---

## 🎨 Philosophy: Take What You Want

By decoupling the **Kernel** from the **Showroom**, Rupa ensures maximum efficiency:

1.  **Terminal Focus**: Build interactive CLIs with `rupa-terminal`.
2.  **Logic Focus**: Run background services with `rupa-engine` (Kernel only).
3.  **Visual Focus**: Build 4K Desktop apps with `rupa-desktop`.
