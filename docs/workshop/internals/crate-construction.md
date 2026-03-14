# Crate Construction Standards 🏗️

This document defines the mandatory standards for creating, splitting, and maintaining crates within the Rupa Framework monorepo. Our goal is to maintain a "Modular-First" ecosystem where every piece is a reusable, high-quality tool.

---

## 1. The Artisan Workshop Tiers

We categorize crates into three distinct tiers to ensure strict boundaries and prevent "God Crate" bloat.

### 1.1 Tier 1: Atomic Materials (The Materials & Tools — Ports & Invariants)
Atomic Materials are the smallest functional units of the framework.
*   **Artisan Identity:** **The Materials & Tools**. Raw materials and sharp chisels.
*   **Technical Identity:** **Ports & Invariants**. Single specialized domains, 100% platform-agnostic.

### 1.2 Tier 2: Composite Assemblies (The Master’s Craft — Kernel & Orchestrator)
Composite Assemblies assemble multiple Atomic Materials into high-level functional modules.
*   **Artisan Identity:** **The Master’s Craft**. The specialized joinery and assembly logic.
*   **Technical Identity:** **Kernel & Orchestrator**. Agnostic technical systems bridging materials to platform needs.

### 1.3 Tier 3: Artisan Showrooms (The Finished Showroom — Adapters & Infrastructure)
Artisan Showrooms pre-assembled for specific business use cases.
*   **Artisan Identity:** **The Finished Showroom**. Polished works displayed for the world.
*   **Technical Identity:** **Adapters & Infrastructure**. Zero-boilerplate target-specific entry points.

---

## 2. When to Split a Crate

Splitting a crate is a strategic decision. A new crate should be created if any of the following conditions are met:

1.  **Circular Dependency:** If two modules within a crate need to refer to each other in a way that creates a loop, they should be split into independent atomic materials.
2.  **Dependency Weight:** If a feature introduces a heavy 3rd-party dependency (e.g., `wgpu` or `taffy`) that isn't needed by all users, it must be isolated.
3.  **Independent Utility:** If a module is useful outside of the UI framework (e.g., using `rupa-signals` for a CLI tool), it should be an Atomic Material.
4.  **Target Divergence:** Code that is specific to one environment (e.g., Web DOM vs. Native GPU) must be separated.

---

## 3. Crate Boundaries & Dependency Rules

To maintain high maintainability, we enforce a strict directional flow of dependencies:

1.  **Atomic Materials (Tier 1)** → Can only depend on other **Atomic Materials**.
2.  **Composite Assembly Assemblies (Tier 2)** → Can depend on **Atomic Materials** and other **Composite Assembly Assemblies**.
3.  **Artisan Showrooms (Tier 3)** → Re-exports **Assemblies** and provides the unified business-ready API.
4.  **Downstream (User Apps)** → Should primarily depend on a **Showroom**, but can opt-into specific **Materials** for specialized builds.

---

## 4. Construction Checklist

Every new crate in the Rupa monorepo must:
- [ ] Have a clear, industry-standard name prefixed with `rupa-`.
- [ ] Implement `serde::Serialize` and `Deserialize` for its core data structures.
- [ ] Follow the **Clean Index** rule (no logic in `mod.rs`).
- [ ] Be documented in `docs/crate-references.md`.
- [ ] Maintain 100% English-only comments and documentation.
