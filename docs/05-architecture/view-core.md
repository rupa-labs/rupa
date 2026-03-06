# Module: View Core (`view.rs`) 🦴

The View Core is the "Anatomical Standard" for the visual part of any component. It consolidates all infrastructure logic into a single, reusable core to eliminate boilerplate.

---

## 🧠 Internal Anatomy

### 1. State Consolidation
Every `ViewCore` stores three essential pieces of metadata:
- **`style`**: The visual blueprint (`RefCell<Style>`).
- **`node`**: The handle to the physical Scene Node (`Cell<Option<SceneNode>>`).
- **`dirty`**: The invalidation flag (`AtomicBool`).

### 2. Composition Logic
Instead of every component implementing its own `dirty` tracking, they **compose** `ViewCore`. This ensures that `mark_dirty()` and `set_node()` behave identically across the entire framework (DRY).

---

## 🗝️ API Anatomy

- `new(style)`: Initializes the visual state.
- `get_style_mut()`: Provides safe, interior-mutability access to the style object.
- `mark_dirty()` / `clear_dirty()`: Manages the re-render signal.

---

## 🛡️ Thread Safety
By using `AtomicBool` for the dirty flag and `RefCell`/`Cell` for internal state, `ViewCore` ensures that components can be manipulated safely within the framework's reactive execution context.
