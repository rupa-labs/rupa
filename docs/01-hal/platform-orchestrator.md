# Module: Platform Orchestrator (`mod.rs`) 🏛️

The Platform Orchestrator is the high-level manager of the Rupaui execution lifecycle. It abstracts the "How" of starting an application, allowing the developer to focus on the "What."

---

## 🧠 Internal Anatomy

### 1. App Struct (The Conductor)
- **Role:** High-level builder.
- **Responsibility:** Orchestrates the bootstrap process, including plugin registration and theme initialization. It holds the root component until the platform runner is ready.

### 2. PlatformCore (The Heart)
- **Role:** Shared State Container.
- **Responsibility:** Composes the **SceneCore (L3)** and tracks global cursor positions. By being composed into both `GuiRunner` and `TuiRunner`, it ensures that UI logic remains identical across backends.

### 3. Redraw Proxy
- **Role:** Communication Bridge.
- **Mechanism:** Provides a `request_redraw()` hook that uses an internal `OnceLock` proxy to signal the active event loop (Winit or Crossterm) without exposing their specific types.

---

## 🗝️ Public API

### `struct App`
- `App::new(name)`: Entry point.
- `.root(component)`: Definining the UI entry.
- `.run()`: Executes the GUI shell.
- `.run_tui()`: Executes the Terminal shell.

---

## 🔄 Interaction Flow
1. **Bootstrap:** `App` initializes L9 (Themes) and L5 (Plugins).
2. **Handover:** `App` transfers the `PlatformCore` to a specific `Runner`.
3. **Loop:** The `Runner` enters an infinite loop, delegating redraws to the hardware.
