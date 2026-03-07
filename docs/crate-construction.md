# Crate Construction Standards 🏗️

This document defines the mandatory standards for creating, splitting, and maintaining crates within the Rupa Framework monorepo. Our goal is to maintain a "Modular-First" ecosystem where every piece is a reusable, high-quality tool.

---

## 1. The Atomic vs. Composite Philosophy

We categorize crates into two distinct types to ensure strict boundaries and prevent "God Crate" bloat.

### 1.1 Atomic Crates (The Atoms)
Atoms are the smallest functional units of the framework.
*   **Responsibility:** Single, specialized domain (e.g., Reactivity, Color Math, ID generation).
*   **Dependencies:** Must only depend on other Atoms or minimal 3rd-party crates.
*   **Platform:** Must be 100% platform-agnostic (`no_std` compatible where possible).
*   **Testing:** Must be fully testable in a headless environment.

### 1.2 Composite Crates (The Assemblies)
Composites assemble multiple Atoms into high-level functional modules.
*   **Responsibility:** Providing a "ready-to-use" feature set (e.g., UI library, Rendering Engine, SSR).
*   **Dependencies:** Orchestrate multiple Atoms. Can depend on platform-specific crates (e.g., `wgpu`, `axum`).
*   **Boundary:** Should not leak internal Atom APIs unless necessary for advanced customization.

---

## 2. When to Split a Crate

Splitting a crate is a strategic decision. A new crate should be created if any of the following conditions are met:

1.  **Circular Dependency:** If two modules within a crate need to refer to each other in a way that creates a loop, they should be split into independent atoms.
2.  **Dependency Weight:** If a feature introduces a heavy 3rd-party dependency (e.g., `wgpu` or `taffy`) that isn't needed by all users, it must be isolated.
3.  **Independent Utility:** If a module is useful outside of the UI framework (e.g., using `rupa-signals` for a CLI tool), it should be an Atom.
4.  **Target Divergence:** Code that is specific to one environment (e.g., Web DOM vs. Native GPU) must be separated.

---

## 3. Crate Boundaries & Dependency Rules

To maintain high maintainability, we enforce a strict directional flow of dependencies:

1.  **Atoms** → Can only depend on **Atoms**.
2.  **Composites** → Can depend on **Atoms** and other **Composites**.
3.  **The Facade (`rupa`)** → Re-exports **Composites** and provides the unified API.
4.  **Downstream (User Apps)** → Should primarily depend on the **Facade**, but can opt-into specific **Atoms** for specialized builds.

---

## 4. Construction Checklist

Every new crate in the Rupa monorepo must:
- [ ] Have a clear, industry-standard name prefixed with `rupa-`.
- [ ] Implement `serde::Serialize` and `Deserialize` for its core data structures.
- [ ] Follow the **Clean Index** rule (no logic in `mod.rs`).
- [ ] Be documented in `docs/crate-references.md`.
- [ ] Maintain 100% English-only comments and documentation.
