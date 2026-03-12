# Rupa Framework Architecture 🏛️

Rupa Framework adopts the **Hexagonal Architecture (Ports & Adapters)** model, seamlessly integrated with a 3-tier structure: **Atomic Materials, Agnostic Kernel, & Artisan Showrooms**. This architecture is engineered to achieve *zero-coupling* between core business logic and platform-specific infrastructure.

---

## 🏗️ The Hexagonal Blueprint: DNA to Showroom

Rupa Framework follows a strict separation of concerns, organized into three layers of responsibility:

### 1. Tier 1: Atomic Materials (The Ports & DNA) 🧱
This layer defines the **DNA** of the system and the **Ports** (Contracts/Interfaces).
*   **Hexagonal Role**: **Ports**. It defines *what* the system can do without worrying about *how* it is implemented.
*   **Key Crates**: `rupa-signals` (Reactivity), `rupa-vnode` (UI Contract), `rupa-base` (Foundational Types).
*   **Rule**: Must not have any dependencies on specific platforms (WGPU, Browser APIs, OS-specific crates).

### 2. Tier 2: Composite Assemblies (The Agnostic Core) 🛠️
This layer is the **Brain** or **Kernel** of the framework, orchestrating various Atoms into functional systems.
*   **Hexagonal Role**: **Core/Logic**. Contains reconciliation logic, plugin orchestration, and application lifecycle management.
*   **Key Crates**: `rupa-core` (Reconciler & Action Bus), `rupa-engine` (Agnostic Kernel).
*   **Rule**: Operates entirely in an agnostic memory space. It invokes **Driven Ports** (like `trait Renderer`) which are fulfilled by the Showroom layer.

### 3. Tier 3: Artisan Showrooms (The Adapters) 🏪
This layer is the **Physical Body** of the application, providing specific implementations for hardware or targets.
*   **Hexagonal Role**: **Adapters**. Connects the Core to the outside world (GPU, DOM, Terminal).
*   **Key Crates**: `rupa-desktop` (WGPU Adapter), `rupa-web` (WASM/DOM Adapter), `rupa-terminal` (Crossterm Adapter).
*   **Rule**: This is where heavyweight, platform-specific dependencies (Winit, WGPU, JS-Sys) are allowed to reside.

---

## 🎨 Philosophy: Take What You Want

This **Hexagonal 3-Tier** structure is what enables Rupa's primary philosophy: **"Use only what you need."** Unlike monolithic frameworks, Rupa is designed as a modular ecosystem where you can pull *Atoms* or *Composites* independently without the weight of the entire *Showroom*.

1.  **Terminal Focus**: Build interactive CLIs using only `rupa-terminal` & `rupa-tui`. Lightweight and fast.
2.  **Logic Focus**: Run background services or automation using only `rupa-engine` & `rupa-signals` (Kernel-only). Zero UI overhead.
3.  **Visual Focus**: Build graphics-intensive 4K Desktop apps with `rupa-desktop` & `rupa-motion`.
4.  **Materialist Focus**: Use only `rupa-signals` as a pure, standalone state management library for any other Rust project.

By decoupling the **Kernel** from the **Showroom**, Rupa ensures maximum efficiency: you never pay the price (in binary size or compile time) for features you don't use.
