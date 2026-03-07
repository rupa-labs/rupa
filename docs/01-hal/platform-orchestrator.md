# Module: Platform Orchestrator (`src/platform/mod.rs`) 🏛️

The Platform Orchestrator manages the application's lifecycle, shared state, and inter-thread communication in an OS-agnostic way.

---

## 🧠 Core Architecture

### 1. App Struct (The Builder)
- **Role:** High-level builder and plugin entry point.
- **Responsibility:** Configures plugins, themes, and root components before handover to a specific platform runner.
- **Plugin Integration:** Provides `add_event_listener()` to register global plugin hooks that can monitor or intercept input events.

### 2. SharedPlatformCore (The Heart)
- **Role:** Thread-Safe State Container.
- **Implementation:** `Arc<RwLock<PlatformCore>>`.
- **Responsibility:**
  - Manages **Geometric Tree**: Integrates `SceneCore (L3)` for hit-testing and layout.
  - Tracks **Interaction State**: Cursor position, Pointer Capture ID, and Keyboard Focus ID.
  - Exposes **Plugin Hooks**: Stores `event_listeners` called by the dispatcher.

### 3. Redraw Strategy
- **Mechanism:** Redraws are managed through a global thread-safe registry (`GLOBAL_REDRAW_PROXY`).
- **Functionality:** `request_redraw()` can be safely called from any thread (including reactive signals) to signal the OS to refresh the frame.

---

## 🗝️ Public API

### `struct App`
- `App::new(name)`: Initialize construction.
- `.root(component)`: Set the UI entry point.
- `.add_event_listener(f)`: Register a plugin hook for input monitoring.
- `.run()`: Start the **GUI Runner**.
- `.run_tui()`: Start the **TUI Runner**.

### `enum PlatformEvent`
- `RequestRedraw`: Siganls the event loop to refresh the current viewport.

---

## 🔄 Lifecycle Logic
1. **Creation:** Developer configures `App`.
2. **Handover:** `App` creates the `SharedPlatformCore` and hands it to a backend Runner.
3. **Execution:** The Runner enters the OS event loop and registers its redraw proxy.
4. **Update:** Input or reactive signals trigger a redraw signal via the proxy.
