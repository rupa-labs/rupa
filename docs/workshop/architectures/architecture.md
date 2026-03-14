# Rupa Core Architecture 🏛️

Rupa Framework is built on the **Atoms and Composites** architecture, a specialized 3-tier adaptation of the **Hexagonal (Ports & Adapters)** model. This structure ensures a high degree of decoupling, enabling the framework to be truly multi-platform and multi-purpose.

---

## 🏗️ The 3-Tier Blueprint

### Tier 1: Atoms (Materials 🧱)
**Artisan Identity**: Pure, stable invariants.
**Technical Context**: **Ports & Invariants**.
This layer defines the DNA of the system—the fundamental types, reactive primitives, and UI contracts that remain constant across all environments. 
- *Rule*: No dependencies on specific platforms or hardware.

### Tier 2: Composites (The Master's Craft 🛠️)
**Artisan Identity**: Specialized logic and assembly.
**Technical Context**: **Kernel & Orchestrator**.
This layer is the "Brain" of the framework. It orchestrates various Atoms into functional systems like reconciliation, application lifecycle management, and plugin systems.
- *Rule*: Operates entirely in an agnostic memory space, interacting with hardware only through abstract Ports.

### Tier 3: Showrooms (The Finished Showroom 🏪)
**Artisan Identity**: The final presentation for specific environments.
**Technical Context**: **Adapters & Infrastructure**.
This layer is the "Physical Body." It provides the hardware-specific implementations (**Adapters**) for the Ports defined in the layers below (e.g., **Desktop Adapter** for WGPU, **Terminal Adapter** for Crossterm).
- *Rule*: This is the only place where platform-specific dependencies are allowed.

---

## 🎨 Architectural Pillars (Macro)

### 1. Zero-Coupling via Ports & Adapters
By defining standardized **Interfaces (Ports)** in Tier 1 and 2, Rupa ensures that the core logic (Composites) never knows the details of the hardware it's running on. This is what allows for "Take what you want" modularity.

### 2. The Universal UI Contract (VNode DNA)
Rupa uses a virtual UI tree (VNodes) as a serializable, platform-agnostic bridge. The **Component Tree** (Logic) produces a **VNode Tree** (Description), which the **Showroom** (The Physical Manifestation) then renders via a specific **Adapter**.

### 3. Fine-Grained Reactivity (The Pulse)
The framework's heartbeat is driven by a Signal-based reactivity engine. This ensures that updates are surgical—only the parts of the system that actually change are ever processed.

### 4. Decoupled Communication (The Action Bus)
Rupa uses an asynchronous **[Action Bus](./action-bus.md)** to allow components and systems to communicate through serializable intents without tight coupling.

### 5. Modular Extensibility (The Plugin System)
The framework is extended through a **[Plugin System](./plugin-system.md)**, allowing specific hardware **Adapters** to "plug" into the agnostic Kernel.

---

## 🚀 The Global Flow

1. **State Change**: A Signal is updated.
2. **Reconciliation**: The Kernel identifies which components need to re-render.
3. **Manifestation**: The Showroom (via its **Adapter**) executes the resulting patches onto the hardware.

*For detailed technical specifications of individual sub-systems, please refer to the specialized documentation in the `architectures/` and `internals/` directories.*
