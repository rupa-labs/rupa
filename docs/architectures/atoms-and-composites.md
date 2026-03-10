# Deep Dive: Atoms & Composites Architecture 🧱

The **Atoms & Composites** architecture is the foundational design pattern of the Rupa Framework. It is what enables Rupa to function as a **modular meta-framework**, serving multiple purposes across different platforms while maintaining strict architectural integrity.

---

## 1. The Core Analogy: Materials vs. Furniture

To understand Rupa, imagine a workshop:

*   **Atoms (Tier 1)** are the **Materials**: Screws, wood planks, glass, and glue. A screw doesn't know it's part of a table; it only knows how to hold two things together.
*   **Composites (Tier 2)** are the **Furniture**: Tables, chairs, and cabinets. A table is "composed" of screws and wood planks to serve a specific purpose (holding your coffee).
*   **The Facade (Tier 3)** is the **Showroom**: A pre-assembled collection where you can get a "Living Room Set" (Desktop App) or a "Kitchen Set" (Web SSR) without needing to know how many screws were used.

---

## 2. Tier 1: The Atoms (The Materials) 🪵

Atoms are independent, low-level crates that handle a **single atomic responsibility**.

### Characteristics:
*   **Strict Independence**: Atoms MUST NOT depend on any Composites or the Facade. They are the "leaves" of the dependency tree.
*   **Zero-Cost Abstractions**: They are engineered for maximum performance. If you only use an Atom, you shouldn't pay the performance price of the whole framework.
*   **Agnostic Nature**: They have no knowledge of "Platforms" (Windows, Linux, Web) or "Renderers" (WGPU, TUI).
*   **Serializable DNA**: Atoms primarily deal with data structures that implement `serde`, allowing them to be sent across language and platform boundaries (e.g., Rust to JS).

### Primary Atoms:
| Atom | Responsibility |
| :--- | :--- |
| **`rupa-signals`** | The "Nervous System". Manages reactive state and dependency tracking. |
| **`rupa-vnode`** | The "Universal Language & DNA". Agnostic Virtual Tree structure **and core Style data models** (Color, Layout, Spacing). |
| **`rupa-support`**| The "Foundation". Math primitives (Vec2), IDs, and unified error types. |

---

## 3. Tier 2: The Composites (The Assemblies) 🛠️

Composites are high-level systems that **assemble multiple Atoms** to provide functional solutions for specific platforms or purposes.

### Characteristics:
*   **System Integrators**: They orchestrate the interaction between Atoms. For example, `rupa-core` takes a `Signal` (Signals Atom), a `Style` (VNode Atom), and produces a `VNode` (VNode Atom).
*   **Purpose-Driven**: Each composite is built for a specific execution target (Native, Web, Server, Mobile).
*   **Pluggable/Swappable**: You can replace a Composite (e.g., swapping `rupa-engine::gui` for `rupa-engine::tui`) without touching your component logic.

### Primary Composites:
| Composite | Composition Logic |
| :--- | :--- |
| **`rupa-core`** | `vnode` + `signals` + `support` -> Reconciliation Engine. |
| **`rupa-ui`** | `core` + `vnode` -> **The UI System**. Houses the **UI Component System** and **UI Utilities System**. |
| **`rupa-engine`** | `core` + `ui` + `wgpu/crossterm` -> Hardware-Accelerated Runtime. |
| **`rupa-server`** | `core` + `ui` + `axum` -> High-Performance SSR Engine. |

---

## 4. Interaction Flow: The Reactive Chain 🔗

The strength of this architecture is how data flows through the tiers:

1.  **State Mutation (Atom)**: A `Signal` in `rupa-signals` is updated.
2.  **Logic Trigger (Composite)**: `rupa-core` detects the change and triggers the `render()` method of a `Component`.
3.  **Snapshot Generation (Atom)**: The component produces a `VNode` tree (from `rupa-vnode`) styled with `Style` objects (from `rupa-vnode`).
4.  **Hardware Execution (Composite)**: `rupa-engine` consumes the `VNode` and translates it into WGPU draw calls or ANSI characters.

---

## 5. Strategic Benefits for Rust Artisans

### A. Multi-Purpose Flexibility
Because Atoms are decoupled, Rupa is a **meta-framework**. You can use it as a full UI toolkit, OR you can just use `rupa-signals` for a CLI backend, OR `rupa-vnode` for a custom game engine.

### B. Headless-First Verification
You can test the entire logic of your UI in a "Headless" environment. By asserting the state of the **VNode Atom**, you can verify your UI's behavior without ever needing to open a window or initialize a GPU.

### C. Scalability & Sustainability (3S Doctrine)
*   **Secure (S1)**: Strict layering prevents circular dependencies and "spaghetti" code.
*   **Sustain (S2)**: Smaller, focused crates are easier to maintain, document, and audit.
*   **Scalable (S3)**: You only import what you need. A server-side tool doesn't need to pull in the heavy `wgpu` dependencies of the GUI engine.

---

## 6. Architectural Mandate: Strict Layering 🚫

To maintain the integrity of the Rupa Framework, the following rule is **absolute**:

> **"Lower Tiers must never import from Higher Tiers."**

Any PR that introduces an import from a Composite into an Atom will be rejected as a violation of the **Secure (S1)** principle. Atoms must remain pure, agnostic materials.
