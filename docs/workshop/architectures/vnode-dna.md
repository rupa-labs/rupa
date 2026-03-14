# VNode Contract System (The DNA) 🌳

The **VNode (Virtual Node)** is the universal language of the Rupa Framework. It is a platform-agnostic, serializable representation of a UI element. Following the **Atoms & Composites** model, VNodes live in Tier 1 as the primary **UI Contract Port**.

---

## 1. The VNode Enum

A `VNode` can represent several distinct types of UI structures:

| Variant | Technical Role | Description |
| :--- | :--- | :--- |
| **`Element`** | Structural | A named element (e.g., `"div"`, `"button"`) with styles, attributes, and children. |
| **`Text`** | Content | Raw string content to be shaped and rendered. |
| **`Fragment`** | Logical | A transparent collection of nodes used to group elements without a wrapper. |
| **`Component`** | Placeholder | A metadata marker for a lazily-resolved Artisan component. |
| **`Empty`** | Null | A sentinel node used for conditional rendering. |

---

## 2. VElement Anatomy

The `VElement` is the most complex unit of the DNA. It contains everything required for an **Adapter** to manifest it physically:

- **Tag**: A semantic string identifying the element type.
- **Style**: An agnostic `Style` object (Layout, Colors, Borders).
- **Attributes**: A key-value map for platform-specific properties.
- **Event Handlers**: Logic-free markers that tell the **Input Dispatcher** to listen for specific intents.
- **Key**: An optional unique identifier for O(N) reconciliation performance.

---

## 3. Agnostic Style Model

The `Style` object within a VNode does not contain physical coordinates. It contains **Rules of Intent**:

- **Flex/Grid Rules**: Described via the **Layout Kernel**, then solved by a **Layout Adapter** (e.g., Taffy).
- **Colors**: Defined in **OKLCH** space for perceptual uniformity.
- **Spacing**: Defined in **Artisan Steps** (logical units) that the **Layout Kernel** translates to Pixels or Cells.

---

## 4. The Lifecycle of a VNode

1. **Generation (Tier 2)**: An Artisan component calls `render()`, producing a fresh VNode tree.
2. **Reconciliation (Tier 2)**: The **Reconciliation Kernel** compares the new tree with the previous one.
3. **Patching (Tier 2)**: A list of `Patch` instructions is generated.
4. **Manifestation (Tier 3)**: A **Physical Adapter** (Desktop, Terminal, etc.) consumes the patches and executes the drawing logic.

---

## 5. Architectural Constraints

- **Purity**: VNodes must never contain platform-specific logic (no WGPU meshes, no DOM references).
- **Serializability**: Every VNode MUST be serializable to JSON to enable SSR and remote rendering protocols.
- **Immutability**: Once a VNode tree is produced, it is treated as an immutable snapshot of the UI at a specific point in time.
