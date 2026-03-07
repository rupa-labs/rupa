# Module: Component Trait (`crates/rupa-core/src/component.rs`) 🧩

The `Component` trait is the core behavioral contract for all UI elements in the Rupa Framework. It defines how a component describes its visual structure and manages its reactive state.

---

## 🧠 Core Architecture

### 1. The Rendering Hook (`render`)
The primary responsibility of a component is to implement the `render()` method:
- **`fn render(&self) -> VNode`**: This method returns a `VNode` tree representing the component's UI. It is called by the framework's reactive engine whenever a signal dependency changes.
- **Agnosticism**: By returning a `VNode` instead of drawing directly, components remain 100% decoupled from the rendering backend (WGPU, TUI, or HTML).

### 2. The Identity (`id`)
Every component instance is assigned a unique `Id` (from `rupa-support`). This ID is used for:
- **Hit-Testing**: Routing input events to the correct component.
- **Diffing**: Tracking component state across frame updates and tree reconciliations.

### 3. Styling (`Stylable`)
Most components also implement the `Stylable` trait, which allows them to carry a `Style` object and accept functional modifiers (e.g., `p(16.0)`, `bg_primary()`).

---

## 🗝️ API Anatomy (Lifecycle & Events)

- **Event Handlers**: Components can intercept interaction events (Click, Hover, Touch) via specialized handler methods or by attaching listeners to the produced `VNode`.
- **Reactivity**: Components do not manually "mark dirty." Instead, they wrap their state in `Signals`. When a signal changes, the framework automatically schedules a re-render of the affected component.

---

## 🔄 Interaction Flow

1.  **State Change**: A `Signal` inside a component is updated.
2.  **Re-render**: The framework calls `component.render()`.
3.  **VNode Output**: A new `VNode` tree is produced.
4.  **Reconciliation**: The `rupa-core` diffing engine compares the new tree with the old one and generates patches for the active renderer.
