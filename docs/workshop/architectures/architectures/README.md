# Architectural Deep Dives 🏛️

This section contains detailed technical specifications for the internal systems of the Rupa Framework.

---

## 🌳 UI Tree & VNode
Understanding the agnostic UI representation.

*   **[VNode Architecture](./vnode.md)**: The universal language of Rupa UI.
*   **[Element Tree](./element-tree.md)**: How nodes are structured into a functional tree.
*   **[Attributes](./attributes.md)**: Metadata and property management for virtual elements.
*   **[Reconciliation](./reconciliation.md)**: The Diff/Patch engine that drives updates.

--- 

## ⚙️ Core Logic & Patterns
Foundational design patterns for components and modules.

*   **[Workshop Tiers](./workshop-tiers.md)**: Detailed breakdown of the 3-tier hexagonal model.
*   **[Component Trait](./component-trait.md)**: The contract for all UI components.
*   **[Logic & View Separation](./logic-and-view.md)**: Decoupling business rules from visual output.
*   **[Module Standard](./module-standard.md)**: Standards for creating new Rupa crates.

--- 

## 🔌 Extensions & Infrastructure
Systems for scaling and extending framework capability.

*   **[Plugin System](./plugin-system.md)**: Architecture for framework-level extensions.
*   **[View Core](./view-core.md)**: The agnostic kernel for managing application state.
*   **[Root & Body](./root-and-body.md)**: Entry points for the UI tree.

--- 

## 🛠️ Debugging & Content
*   **[Debug Mode](./debug-mode.md)**: Internal instrumentation for development.
*   **[Content Driven](./content-driven.md)**: Architecture for document-centric applications.
