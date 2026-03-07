# Engineering Standards 🛠️

This document defines the mandatory engineering principles, patterns, and conventions followed in the Rupaui framework. Adherence to these standards ensures a modular, maintainable, and platform-agnostic codebase.

---

## 1. Core Architectural Pillars

### Separation of Concerns (SOC)
Every module must have a single, well-defined responsibility.
- **Logic vs. View:** Components must separate their "brain" (state/events) from their "body" (rendering/layout).
- **Platform Agnosticism:** High-level logic (Layers 3-9) must never depend on hardware-specific details (Layers 1-2).

### DRY (Don't Repeat Yourself) via Composition
Avoid logic duplication by centralizing shared functionality.
- **Composition over Inheritance:** Use "Cores" (e.g., `PlatformCore`, `RenderCore`) and compose them into specific implementations instead of using deep inheritance hierarchies.
- **Trait-Based Behavior:** Define common capabilities (e.g., `Stylable`, `Renderer`) through traits.

### Dependency Inversion (Third-Party Wrapping)
Never let an external library leak its API directly into Rupaui's core logic.
- **Rule:** All 3rd-party crates (Winit, WGPU, Crossterm, Taffy) must be wrapped in internal abstractions.
- **Benefit:** Allows swapping underlying libraries (e.g., replacing Winit with SDL2) by only modifying the wrapper.

---

## 2. Framework Patterns

### The Logic & View Pattern
Standard for all UI Elements in `src/elements/`.
1.  **Logic Struct:** Pure data, signals, and event handlers.
2.  **View Struct:** Styling metadata, layout nodes, and paint instructions.
3.  **Bridge Struct:** The public component that implements `Component` and `Stylable`.

### The Agnostic Bridge
- Layers 3 through 9 communicate using a "Universal Language" (`InputEvent`, `trait Renderer`).
- Implementation details are isolated in Layer 1 (HAL) and Layer 2 (Rendering).

---

## 3. Coding Conventions

### Naming Conventions
- **Semantics First:** Names should describe *what* an object represents or *why* it exists, not *how* it is implemented.
- **Conciseness:** Prefer short, impactful names (e.g., `GuiRunner` instead of `RupauiGraphicalApplicationRunner`).
- **Standard Suffixes:**
    *   `Core`: Internal shared state (Composition).
    *   `Logic`: Component state engine.
    *   `View`: Component rendering engine.
    *   `Runner`: Platform-specific execution shell.

### Modularity
- **One Module, One Responsibility:** Avoid "God Files". If a module exceeds its scope, break it down into sub-modules (e.g., `src/style/modifiers/` split by functional domain).
- **Clean Indices (`mod.rs`):** All `mod.rs` files MUST be clean indices. They should only contain module declarations (`pub mod ...`) and re-exports (`pub use ...`). NO implementation logic, constants, or traits should be defined directly within a `mod.rs` file.
- **Flattened Re-exports:** Use `pub use` in `mod.rs` to keep user imports shallow (1-level deep via `prelude`).


---

## 4. State & Reactivity Standards

- **Signal-First Mutation:** Any state that affects the visual output must be wrapped in a `Signal<T>`.
- **Fine-Grained Updates:** Components should only be marked as `dirty` if a signal they depend on changes.
- **Immutable Logic:** Logic structs should prefer immutability, using interior mutability (`Cell`, `RefCell`) or Signals only where necessary for performance or reactivity.

---

## 5. Documentation Standard

- **1:1 Mapping:** Every physical source file (`.rs`) should ideally have a corresponding documentation file (`.md`) explaining its technical role.
- **Transparency:** No "magic". Document the execution flow and data propagation clearly to lower the barrier for new contributors.
