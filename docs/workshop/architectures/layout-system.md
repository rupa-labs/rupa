# Layout System Architecture 📏

Rupa Framework provides a unified, cross-platform layout engine that bridges the gap between pixel-precision (GUI) and character-grid (TUI) environments.

---

## 1. The Interface-First Design

Following the **Port Mandate**, Rupa's layout system is decoupled from any specific implementation via the `LayoutSolver` interface.

- **`LayoutSolver` (Port)**: A trait that defines how to solve tree-based layout calculations.
- **`TaffySolver` (Adapter)**: Our current implementation using the **Taffy** engine (Flexbox & CSS Grid).
- **`LayoutEngine` (Orchestrator)**: Manages the solver and performs physical coordinate translation.

---

## 2. Dual-Mode Geometry

To ensure consistent visuals across hardware, Rupa uses a **Physical Translation Layer**:

### LayoutMode::Pixel (GUI)
- **Target**: Desktop, Web, Mobile.
- **Precision**: 32-bit floating point.
- **Behavior**: Direct mapping of Taffy's results to the rendering coordinates.

### LayoutMode::Cell (TUI)
- **Target**: Terminal, CLI.
- **Precision**: Integer-based characters (cells).
- **Behavior**: Taffy results are **rounded to the nearest integer**, ensuring that borders and padding always align with the terminal's character grid.

---

## 3. The Execution Flow

1. **Build**: The `LayoutEngine` builds an abstract tree for the solver from the `VNode` tree.
2. **Solve**: The solver (e.g., `TaffySolver`) calculates dimensions based on constraints (Width/Height).
3. **Physicalize**: The `LayoutEngine` extracts the results and applies `LayoutMode` rounding logic.
4. **Scene Update**: The resulting `SceneNode` tree is used for hit-testing and rendering.
