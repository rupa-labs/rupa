# Component: Button 🔘

The `Button` is the primary interactive element in Rupaui. It demonstrates the full power of the **Logic & View** pattern, separating high-level interaction state from low-level GPU/Terminal rendering.

---

## 🧠 Internal Anatomy

### 1. ButtonLogic (The Brain)
Responsible for state management and interaction rules.
- **State:** Tracks `disabled` and `is_loading` via Signals.
- **Rules:** Click events are ignored if the button is disabled or in a loading state.
- **Context:** Does not know about WGPU or Taffy.

### 2. ButtonView (The Body)
Responsible for visual representation.
- **Infrastructure:** Composes `ViewCore` to manage its Taffy `NodeId` and `Style`.
- **Mapping:** Translates `ButtonSize` into physical padding and `Variant` into background colors.
- **Agnosticism:** Implements different drawing paths for GUI (rects) and TUI (box-drawing characters).

---

## 🗝️ Public API (Usage)

### Constructor
- `Button::new(label)`: Initializes the component with a string label.

### Semantic Modifiers
| Method | Description |
| :--- | :--- |
| `.variant(v)` | Sets visual intent (`Primary`, `Danger`, etc). |
| `.size(s)` | Scales the button according to DNA Visual standards. |
| `.on_click(f)` | Attaches a closure to be executed on press. |

---

## 🎨 Visual Identity
Since it implements `Stylable`, users can override any part of the `ButtonView` using functional utilities:
```rust
Button::new("Save")
    .style((bg(Color::Blue(500)), rounded(4.0)))
```

---

## ⚡ Interaction Signals
- **Inputs:** Listens for `PointerButton` (Click) and `PointerMove` (Hover).
- **Outputs:** Triggers a `request_redraw` whenever internal state changes.
