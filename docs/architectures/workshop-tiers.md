# Deep Dive: Atomic Materials & Composite Assemblies 🧱

The **Atomic Materials & Composite Assemblies** architecture is the foundational design pattern of the Rupa Framework. It is what enables Rupa to function as a **modular meta-framework**, serving multiple purposes across different platforms while maintaining strict architectural integrity.

---

## 1. The Artisan Workshop Analogy

To understand Rupa, imagine an artisan's workshop:

*   **Tier 1: Atomic Materials** 🧱 are the **Raw Materials**: Wood planks, screws, glass, and leather. A screw doesn't know it's part of a high-end chair; it only knows how to join two pieces of material.
*   **Tier 2: Composite Assemblies** 🛠️ are the **Furniture Frameworks**: Table frames, drawer mechanisms, and seat structures. These are assembled from Materials to serve a functional purpose.
*   **Tier 3: Artisan Showrooms** 🏪 are the **Curated Sets**: A "Rustic Living Room" or "Modern Office" set. These are pre-composed assemblies ready for immediate use by the end customer.

---

## 2. Tier 1: Atomic Materials 🧱

Atomic Materials are independent, low-level crates that handle a **single atomic responsibility**.

### Primary Atomic Materials:
| Atomic Material | Responsibility |
| :--- | :--- |
| **`rupa-signals`** | The "Nervous System". Manages reactive state and dependency tracking. |
| **`rupa-vnode`** | The "Universal Language & DNA". Agnostic virtual tree and core style data. |
| **`rupa-support`**| The "Foundation". Math primitives (Vec2), IDs, and common Errors. |
| **`rupa-auth`** | The Identity. Reactive authentication and RBAC. |
| **`rupa-store`** | The Memory. "Storage as a Signal" bridge. |

---

## 3. Tier 2: Composite Assemblies 🛠️

Composite Assemblies are high-level systems that **assemble multiple Atomic Materials** to provide functional, platform-specific solutions.

### Primary Composite Assemblies:
| Composite Assembly | Composition Logic |
| :--- | :--- |
| **`rupa-core`** | `vnode` + `signals` + `support` -> Reconciliation Engine. |
| **`rupa-ui`** | `core` + `vnode` -> The UI System (Components + Utilities). |
| **`rupa-engine`** | `core` + `ui` + `wgpu/crossterm` -> Hardware-Accelerated Runtime. |

---

## 4. Tier 3: Artisan Showrooms 🏪

Showrooms are specialized artisan showrooms that provide a **zero-boilerplate entry point** for specific business use cases.

### Available Showrooms:
- **`rupa-desktop`**: Tailored for native GPU applications.
- **`rupa-web`**: Optimized for WASM and browser frontends.
- **`rupa-fullstack`**: The comprehensive, all-in-one universal bundle.

---

## 5. Architectural Mandate: Strict Layering 🚫

To maintain the integrity of the Rupa Framework, the following rule is **absolute**:

> **"Lower Tiers must never import from Higher Tiers."**

Any contribution that introduces an import from a **Composite Assembly** into an **Atomic Material** will be rejected as a violation of the **Secure (S1)** principle.
