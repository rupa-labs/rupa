# Rupa Framework Engineering Standards 🛠️

This document defines the mandatory engineering principles and foundational strategies for the Rupa Framework. Adherence to these standards is critical for maintaining a high-performance, sustainable, and scalable system.

---

## 1. Core Principles: The 3S Doctrine

All engineering activities SHALL be evaluated against the 3S doctrine:

*   **Secure (S1):** Protection of integrity, boundaries, contracts, and failure semantics. Security overrides architectural flexibility.
*   **Sustain (S2):** Maintainability, semantic clarity, documentation completeness, and reduced cognitive load for developers.
*   **Scalable (S3):** Structural modularity, controlled dependency growth, and predictable performance under expansion.

---

## 2. Architectural Governance

### 2.1 Mandatory Separation of Concerns (SOC)
The system SHALL maintain strict logical isolation across layers:
*   **Domain Layer**: Pure business rules with zero I/O or framework dependencies.
*   **Application Layer**: Use-case orchestration and transaction coordination.
*   **Infrastructure Layer**: Physical implementations (File systems, APIs, Databases).
*   **Interface Layer**: Delivery mechanisms (HTTP, CLI, UI).

### 2.2 Hexagonal Architecture (Ports & Adapters)
*   **Tier 1: Atomic Materials (The Materials & Tools)**: Act as **Ports & Invariants** (Contracts/Traits).
*   **Tier 2: Composite Assemblies (The Master’s Craft)**: Act as the **Kernel & Orchestrator** (Logic).
*   **Tier 3: Artisan Showrooms (The Finished Showroom)**: Act as **Adapters & Infrastructure** (Platform-specific implementations).

---

## 3. Dependency Governance

*   **Abstractions over Concretes**: Modules SHALL depend on abstractions (traits), not concrete implementations.
*   **Dependency Inversion**: High-level components SHALL NOT depend on low-level components; both SHALL depend on abstractions.
*   **Zero-Cost Abstractions**: Utilize Rust's generics and traits to ensure abstractions impose no runtime overhead.

---

## 4. Clean Code Standards

*   **Artisan Naming**: Identifiers SHALL express intent, not implementation detail. Use short, clear names within their respective namespaces.
*   **Single Responsibility**: Functions SHALL perform one task. Classes/Structs SHALL represent a single axis of change.
*   **DRY Principle**: Business rules, validation logic, and constants SHALL be defined once in a single authoritative source.

---

## 5. Test-Driven Development (TDD)

Development SHALL follow the **Red-Green-Refactor** cycle:
1.  **Red**: Write a failing test to define behavior or boundaries.
2.  **Green**: Write the minimal code necessary to pass the test.
3.  **Refactor**: Optimize structure while maintaining behavior and ensuring tests stay green.

*Tests are treated as executable specifications that document the system's behavior.*

---

## 6. Documentation Standards (The Rustdoc Layer)

Rupa Framework mandates the use of **Rustdoc** for all public APIs to ensure ease of use for Artisans.

### 6.1 Documentation Syntax
*   **Item Documentation (`///`)**: Use triple slashes to document structs, functions, traits, and fields.
*   **Module Documentation (`//!`)**: Use at the first line of `lib.rs` or `mod.rs` to explain the module's purpose.

### 6.2 Mandatory Content Structure
Every public function documentation SHOULD include the following sections where relevant:
*   **`# Examples`**: Valid usage examples (automatically verified as *doc-tests* by the compiler).
*   **`# Errors`**: Explanation of when the function returns an `Err` variant.
*   **`# Panics`**: Explanation of conditions that trigger a thread panic.

---

## 7. Module Hygiene

*   **Clean Indices**: `mod.rs` files SHALL only contain declarations (`pub mod`) and re-exports (`pub use`). Implementation logic is prohibited in index files.
*   **Flat Prelude**: Keep user imports shallow through a centralized `prelude` module.
