# Application Bootstrap: App Orchestration 🚀

The Composition Layer is where the developer takes command of the Rupaui framework. It provides the high-level orchestration required to turn a tree of components into a living, interactive application.

---

## 🧠 Internal Anatomy

### 1. The App Builder
- **Role:** Orchestrator.
- **Responsibility:** Manages the registration of **Plugins (L5)** and the initialization of the **Design System (L9)**. It holds the "Blueprints" until the platform-specific runner is invoked.

### 2. Handover Mechanism
When `.run()` or `.run_tui()` is called, the `App` performs a **Handover**:
1. Executes `bootstrap()` to sync global themes.
2. Builds all registered plugins.
3. Transfers ownership of the `Root Component` to the **Platform Runner (L1)**.

---

## 🗝️ Public API

### `struct App`
- `App::new(name)`: Sets the OS-level application name.
- `.root(component)`: Defines the entry point of the element tree.
- `.run()`: Bootstraps the GUI environment (Winit/WGPU).
- `.run_tui()`: Bootstraps the Terminal environment (Crossterm).

---

## 🔄 Interaction Flow
- **L8 -> L1:** Triggers the platform-specific event loop.
- **L8 -> L5:** Invokes plugin build hooks.
- **L8 -> L9:** Initializes the DNA Visual theme engine.
