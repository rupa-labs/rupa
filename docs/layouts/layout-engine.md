# Module: Layout Engine (`layout.rs`) 📐

The Layout Engine is the mathematical unit of Layer 3. It serves as the formal wrapper for the **Taffy** layout algorithm, providing a bridge between artisan styles and physical coordinates.

---

## 🧠 Internal Anatomy

### 1. Taffy Integration
- **Role:** Dependency Wrapping.
- **Mechanism:** Maintains an internal `TaffyTree`. It isolates Taffy's complex API from the rest of Rupa Framework, ensuring that we can swap layout providers in the future without breaking the framework.

### 2. Recursive Building
During the `compute()` phase, the engine:
1. Clears the previous tree.
2. Recursively calls `component.layout()`.
3. Links the resulting `NodeId`s according to the component hierarchy.
4. Executes the final `taffy.compute_layout()` pass.

---

## 🗝️ API Anatomy

- `new()`: Initializes the Taffy workspace.
- `compute(root, measurer, w, h)`: The high-level resolution command. It requires a `TextMeasurer` (L2) to account for dynamic content sizing.

---

## 🔄 Interaction Flow
- **L3 (Layout Engine) -> L5 (Component):** Triggers the tree-wide `layout()` calls.
- **L3 (Layout Engine) -> L2 (Measurer):** Requests real-time text dimensions.
