# Conceptual Guide: Logic & View Pattern 🧠🎭

The **Logic & View** pattern is the mandatory architectural standard for all components in Rupa Framework. It enforces a strict **Separation of Concerns (SOC)** by splitting an element into its "Brain" and its "Body."

---

## 🏗️ The Architectural Split

Every component is composed of three primary structures:

### 1. The Logic (The Brain)
- **Role:** Pure UI logic and reactive state.
- **Contents:** `Signals`, `Memos`, data validation, and event handlers.
- **Rule:** Must **NEVER** import `wgpu`, `taffy`, or any rendering-specific crate.
- **Benefit:** Can be unit-tested in a pure CLI environment without hardware.

### 2. The View (The Body)
- **Role:** Visual infrastructure and hardware detail.
- **Contents:** `Style` object, Taffy `NodeId`, and the `dirty` flag.
- **Rule:** Is purely reactive; it reads from the Logic to decide *how* to draw.
- **Benefit:** Allows swapping the visual implementation (e.g., GUI vs. TUI) without changing the component's behavior.

### 3. The Bridge (The Component)
- **Role:** The public-facing struct that unifies Logic and View.
- **Contents:** Implements the `Component` and `Stylable` traits.
- **Responsibility:** Delegates `layout()` and `paint()` calls to the View while routing input events to the Logic.

---

## 🗝️ Standard Implementation Pattern

```rust
// 1. Logic
pub struct MyButtonLogic { pub label: Signal<String>, ... }

// 2. View
pub struct MyButtonView { pub style: RefCell<Style>, ... }

// 3. Bridge
pub struct MyButton { pub logic: MyButtonLogic, pub view: MyButtonView }

impl Component for MyButton {
    fn layout(...) { self.view.compute_layout(..., &self.logic) }
    fn paint(...) { self.view.render(..., &self.logic) }
    fn on_click(...) { self.logic.handle_click(...) }
}
```

---

## 🚀 Why This Pattern?

1.  **Testability:** You can test that a `Switch` toggles its state without needing to verify if a pixel turned green.
2.  **Platform Agnosticism:** The same `InputLogic` works perfectly in a 4K Desktop app and an SSH Terminal session.
3.  **Maintainability:** Prevents the creation of "God Components" where rendering code is mixed with complex business logic.
