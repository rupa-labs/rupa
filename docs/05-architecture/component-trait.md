# Module: Component Trait (`component.rs`) 🧩

The `Component` trait is the behavioral contract for all UI elements. It defines the formal lifecycle and the Agnostic Bridge between UI logic and physical rendering.

---

## 🧠 Internal Anatomy

### 1. Hierarchy Methods
- **`children()`**: Returns a vector of trait objects (`&dyn Component`). This enables the framework to perform recursive traversals without knowing the concrete types of the children.

### 2. Geometric Methods
- **`layout()`**: The link to Layer 3. It receives a `TextMeasurer` from Layer 2 to ensure content-aware sizing.
- **`paint()`**: The link to Layer 2. It accepts a `&mut dyn Renderer`, allowing the same component to be drawn on GPUs or Terminals.

### 3. State Management
- **`mark_dirty()` / `is_dirty()`**: Standard hooks used by the framework to skip unnecessary layout calculations, directly tied to the **Fine-Grained Updates** philosophy.

---

## 🗝️ API Anatomy (Lifecycle Hooks)

- **`on_click`, `on_scroll`, `on_drag`**: Interactive hooks called by the `InputDispatcher`.
- **`render()`**: A high-level logical hook (default implementation provided) for components to define their internal structure.

---

## 🔄 Interaction Flow
- **L5 (Component) -> L3 (Geometric Scene):** Participates in the tree-building process.
- **L5 (Component) -> L4 (Reactivity):** Listens to signals to trigger invalidation.
