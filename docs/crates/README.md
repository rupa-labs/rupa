# Rupa Atomic Materials: The Atoms 🧬

Welcome to the **Atoms** gallery. In the **Atoms and Composites** architecture, Atoms represent Tier 1—the smallest, most stable, and platform-agnostic building blocks of the Rupa Framework.

---

## 🏛️ The Identity of an Atom
Atoms are defined by their **Purity** and **Stability**. They act as the **DNA** (Data & Invariants) and the **Ports** (Contracts) of the system.
*   **100% Platform Agnostic**: No dependencies on OS-specific or hardware-specific crates.
*   **Highly Decoupled**: Can be used independently or assembled into **Composites**.
*   **TDD Mandated**: Every Atom is verified in a headless environment.

---

## 🧱 Atoms Directory

### 🧬 Core DNA
Foundational logic that powers the framework's reactive and visual nature.
*   [**`rupa-signals`**](./rupa-signals.md): Fine-grained reactivity heartbeat.
*   [**`rupa-vnode`**](./rupa-vnode.md): Universal UI contract and DNA.
*   [**`rupa-base`**](./rupa-base.md): Standard types (Colors, Vectors, IDs).
*   [**`rupa-motion`**](./rupa-motion.md): Physics-based and eased animation engine.

### 🔌 Infrastructure Ports
Abstract contracts for external system integration.
*   [**`rupa-auth`**](./rupa-auth.md): Identity and Session management.
*   [**`rupa-store`**](./rupa-store.md): Reactive state persistence backends.
*   [**`rupa-net`**](./rupa-net.md): Async networking and resource management.
*   [**`rupa-broadcast`**](./rupa-broadcast.md): Global reactive event bus.
*   [**`rupa-assets`**](./rupa-assets.md): Non-blocking resource loading and caching.

### ⚙️ Systemic Atoms
Crates that provide systemic orchestration and coordination.
*   [**`rupa-queue`**](./rupa-queue.md): Background task and concurrency management.
*   [**`rupa-context`**](./rupa-context.md): Dependency injection and scoped registries.
*   [**`rupa-telemetry`**](./rupa-telemetry.md): Framework-wide diagnostic hub (Logs/Metrics/Traces).

### 🎨 Interaction & Media Atoms
Building blocks for user interfaces and graphics.
*   [**`rupa-forms`**](./rupa-forms.md): Reactive validation and state management for data entry.
*   [**`rupa-canvas`**](./rupa-canvas.md): Low-level hardware-accelerated drawing instructions.
*   [**`rupa-i18n`**](./rupa-i18n.md): Reactive globalization and translation services.
*   [**`rupa-a11y`**](./rupa-a11y.md): Semantic accessibility and screen reader ports.

### 🧪 Quality & DX Atoms
Tools for ensuring artisan excellence.
*   [**`rupa-test`**](./rupa-test.md): Headless testing and snapshot validation.
*   [**`rupa-console`**](./rupa-console.md): Low-level terminal infrastructure and CLI primitives.

---

## 🛠️ From Atoms to Composites
Once you have mastered the Atoms, see how they are assembled into powerful systems in the [**Composites Guide**](../architecture.md#tier-2-composites).
