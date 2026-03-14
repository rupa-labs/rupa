# Input System Architecture 🎮

Rupa Framework abstracts hardware-specific inputs into a unified, semantic model called **Intents**. This ensures that components behave consistently across GUI (Mouse/Touch) and TUI (Keyboard/Focus).

---

## 1. The Unified Input Model

Input is categorized into three primary pillars:

### A. Pointer Events (Positional)
Represent inputs with spatial coordinates.
- **Hardware**: Mouse, Touchscreen, Stylus.
- **Actions**: `Down`, `Up`, `Move`, `Hover`, `Scroll`.
- **Coordinate Space**: Floating-point pixels (GUI) or Integer cells (TUI).

### B. Focus & Navigation (Logical)
Represent logical movement through the UI tree.
- **Hardware**: Keyboard (Tab, Arrows), Gamepad.
- **Actions**: `Enter` (Focus In), `Leave` (Focus Out), `Next/Prev` (Tab cycling), `Directional` (Arrow navigation).

### C. Raw Key Events
Direct access to keyboard states for text input or specialized hotkeys.

---

## 2. Intent-Based Event Listeners

To maintain platform agnosticism, Artisans should prioritize **Intent Listeners** over raw input handlers:

| Intent | Trigger (GUI) | Trigger (TUI) |
| :--- | :--- | :--- |
| **`on_submit`** | Left Click | Enter / Space |
| **`on_cancel`** | Click Outside | Escape |
| **`on_select`** | Hover | Tab / Arrow Focus |

---

## 3. The Dispatcher Pipeline

1. **Native Event Capture**: The Showroom (Tier 3) captures an OS event (e.g., `winit::WindowEvent`).
2. **Translation**: The Showroom maps it to a Rupa `InputEvent`.
3. **Hit-Testing**: The `InputDispatcher` uses the `SceneCore` to find the target `VNode` based on coordinates or current focus.
4. **Bubbling**: The event bubbles up the tree, triggering relevant handlers.
