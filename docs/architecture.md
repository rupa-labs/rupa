# Rupa Framework Architecture 🏛️

Rupa Framework is built on the **Atoms and Composites** architecture, a unique adaptation of the **Hexagonal (Ports & Adapters)** model. This 3-tier structure is engineered to achieve *zero-coupling* between core business logic and platform-specific infrastructure.

---

## 🏗️ The Atoms and Composites Blueprint

Rupa Framework follows a strict separation of concerns, organized into three layers of responsibility:

### 1. Tier 1: Atoms (The Materials & Tools — Ports & Invariants) 🧱
This layer defines the **DNA** of the system and the **Ports** (Contracts/Interfaces).
*   **Artisan Identity**: **The Materials & Tools**. Pure, stable invariants and raw materials.
*   **Technical Identity**: **Ports & Invariants**. Defines *what* the system can do without any platform-specific side effects.
*   **Key Crates**: `rupa-signals` (Reactivity), `rupa-vnode` (UI Contract), `rupa-base` (Foundational Types).
*   **Rule**: Must not have any dependencies on specific platforms (WGPU, Browser APIs, OS-specific crates).

### 2. Tier 2: Composites (The Master’s Craft — Kernel & Orchestrator) 🛠️
This layer is the **Brain** or **Kernel** of the framework, orchestrating various Atoms into functional systems.
*   **Artisan Identity**: **The Master’s Craft**. The specialized logic and techniques used to assemble Atoms.
*   **Technical Identity**: **Kernel & Orchestrator**. Contains reconciliation logic, plugin orchestration, and application lifecycle management.
*   **Key Crates**: `rupa-core` (Reconciler & Action Bus), `rupa-engine` (Agnostic Kernel).
*   **Rule**: Operates entirely in an agnostic memory space. It invokes **Driven Ports** (like `trait Renderer`) which are fulfilled by the Showroom layer.

### 3. Tier 3: Showrooms (The Finished Showroom — Adapters & Infrastructure) 🏪
This layer is the **Physical Body** of the application, providing specific implementations for hardware or targets.
*   **Artisan Identity**: **The Finished Showroom**. The final presentation of the work for specific environments.
*   **Technical Identity**: **Adapters & Infrastructure**. Connects the Core to the outside world (GPU, DOM, Terminal).
*   **Key Crates**: `rupa-desktop` (WGPU Adapter), `rupa-web` (WASM/DOM Adapter), `rupa-terminal` (Crossterm Adapter).
*   **Rule**: This is where heavyweight, platform-specific dependencies (Winit, WGPU, JS-Sys) are allowed to reside.

---

## 🎨 Philosophy: Take What You Want

This **Atoms and Composites** structure is what enables Rupa's primary philosophy: **"Use only what you need."** Unlike monolithic frameworks, Rupa is designed as a modular ecosystem where you can pull *Atoms* or *Composites* independently without the weight of the entire *Showroom*.

1.  **Terminal Focus**: Build interactive CLIs using only `rupa-terminal` & `rupa-tui`. Lightweight and fast.
2.  **Logic Focus**: Run background services or automation using only `rupa-engine` & `rupa-signals` (Kernel-only). Zero UI overhead.
3.  **Visual Focus**: Build graphics-intensive 4K Desktop apps with `rupa-desktop` & `rupa-motion`.
4.  **Materialist Focus**: Use only `rupa-signals` as a pure, standalone state management library for any other Rust project.

By decoupling the **Kernel** from the **Showroom**, Rupa ensures maximum efficiency: you never pay the price (in binary size or compile time) for features you don't use.
