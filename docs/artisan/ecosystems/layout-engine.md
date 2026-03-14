# Module: Layout Engine (`crates/rupa-engine/src/scene/layout.rs`) 📐

The Layout Engine is the mathematical unit of the Scene Graph. It serves as the formal wrapper for the **Taffy** layout algorithm, providing a bridge between the styling system and physical coordinates.

---

## 🧠 Internal Anatomy

### 1. Taffy Integration
- **Role:** Dependency Wrapping.
- **Mechanism:** Maintains an internal `TaffyTree`. It isolates Taffy's complex API from the rest of the Rupa Framework, ensuring that we can swap layout providers in the future without breaking the core engine.

### 2. Recursive Building
During the `compute()` phase, the engine:
1. Clears or updates the previous tree based on the VNode patch instructions.
2. Recursively registers layout nodes and styles for each active VNode.
3. Links the resulting `NodeId`s according to the visual hierarchy.
4. Executes the final `taffy.compute_layout()` pass.

---

## 🗝️ API Anatomy

- **`new()`**: Initializes the Taffy workspace.
- **`compute(root, measurer, w, h)`**: The high-level resolution command. It requires a `TextMeasurer` from the text rendering engine to account for dynamic content sizing.

---

## 🔄 Interaction Flow
- **Layout Engine -> Scene Node:** Updates physical coordinates based on Taffy results.
- **Layout Engine -> Text Measurer:** Requests real-time text dimensions during the layout pass.
