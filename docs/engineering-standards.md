# Rupa Framework Engineering Standards 🛠️

This document defines the mandatory engineering principles and foundational strategies for the Rupa Framework. Adherence to these standards is critical for maintaining a high-performance, scalable, and cross-platform ecosystem.

---

## 1. Core Architectural Pillars (The Doctrine)

### 1.1 Zero-Cost Abstractions
Rupa Framework leverages Rust's type system to ensure that abstractions do not impose a runtime penalty.
*   **Generics over Trait Objects:** Prefer `impl Trait` for internal calculations, styling, and data processing to allow compiler inlining.
*   **Surgical Dynamic Dispatch:** Use `dyn Trait` only at system boundaries (e.g., `Component` or `Renderer` interfaces) where flexibility is required at the cost of slight overhead.

### 1.2 Unified Virtual Tree (VNode Architecture)
To support both GPU-based (WGPU) and DOM-based (Web) rendering, Rupa utilizes a shared intermediary structure: the **VNode**.
*   **Agnostic Interface:** Components describe their intent via VNodes.
*   **Multi-Pipeline Rendering:**
    *   **Native Pipeline:** VNode -> Taffy (Layout) -> WGPU/TUI (Paint).
    *   **Web Pipeline:** VNode -> HTML String (SSR) or DOM Elements (Client).
*   **Separation of Concerns:** Components must never know how they are being rendered.

### 1.3 Async-First & Reactive Integrity
Modern UIs must be non-blocking and responsive.
*   **Async Dispatching:** The event system and reactivity engine must be compatible with Rust's `async/await` ecosystem (Tokio/WASM).
*   **Fine-Grained Reactivity:** UI updates are side-effects of `Signal<T>` changes, ensuring that only the necessary parts of the tree are recalculated.

---

## 2. Platform & Ecosystem Strategy

### 2.1 Pluggable & Modular Design
Rupa follows a "Plugin-First" philosophy. Core features should be decoupled into independent modules.
*   **Feature Flags:** Use Cargo features to allow users to opt-out of heavy dependencies (e.g., `wgpu`, `ssr`).
*   **Trait-Based Extensibility:** Systems like Routing, State Management, and Animations must be injectable via the `Plugin` trait.

### 2.2 Polyglot & Cross-Platform Integrity
The framework is designed to bridge the gap between Rust and the JavaScript ecosystem.
*   **Serialization DNA:** All core data structures (Style, Events, Layout) must implement `serde::Serialize` and `serde::Deserialize`.
*   **Universal ABI:** Maintain a clean FFI/WASM boundary for Node.js, Bun, and Browser interoperability.
*   **Platform Agnosticism:** Layers 3-9 (Core, UI, Composition) must remain 100% free of OS-specific or hardware-specific code.

### 2.3 Strict Diagnostics & Transparency ("No Magic" Rule)
Developer Experience (DX) is as important as performance.
*   **Diagnostic Center:** Failures in rendering, layout, or event propagation must report clear, actionable errors with source context.
*   **Fail-Safe Philosophy:** Use typed errors (`thiserror`) and avoid silent failures or placeholders.

---

## 3. Development Lifecycle

### 3.1 TDD & Empirical Verification
*   **Test-First:** New features or bug fixes must be preceded by an automated test.
*   **Headless Validation:** UI components should be validated in headless environments before being tested in graphical runners.

### 3.2 Documentation Parity (Sync or Sink)
*   **1:1 Mapping:** Every technical implementation in `crates/` must have a corresponding architectural explanation in `docs/`.
*   **Transparency:** No hidden logic. The execution flow from Input -> Signal -> VNode -> Render must be explicit and documented.

---

## 4. Coding Conventions

### 4.1 Naming & Semantics
*   **Intent over Implementation:** Names describe *why* an object exists (e.g., `ArtisanButton`), not how it works (e.g., `WgpuClickableRect`).
*   **Standard Suffixes:**
    *   `Core`: Internal shared state.
    *   `Logic`: Component state engine (The Brain).
    *   `View`: Component visual description (The Body).
    *   `Runner`: Platform-specific execution shell.

### 4.2 Module Hygiene
*   **Clean Indices:** `mod.rs` files must only contain declarations (`pub mod`) and re-exports (`pub use`). Implementation logic is prohibited in index files.
*   **Flat Prelude:** Keep user imports shallow through a centralized `prelude` module.
