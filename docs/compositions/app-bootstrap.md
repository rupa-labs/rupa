# Application Bootstrap: App Orchestration 🚀

The Composition Layer is where the developer takes command of the Rupa Framework framework. It provides the high-level orchestration required to turn a tree of components into a living, interactive application.

---

## 🧠 Internal Anatomy

### 1. The App Builder
- **Role:** Orchestrator.
- **Responsibility:** Manages the registration of **Plugins** and the initialization of the **Design System**. It holds the "Blueprints" until the platform-specific runner is invoked.

### 2. Handover Mechanism
When `.run()`, `.run_tui()`, or `.run_mobile()` is called, the `App` performs a **Handover**:
1. Executes `bootstrap()` to sync global themes.
2. Builds all registered plugins.
3. Transfers ownership of the `Root Component` to the **Platform Runner** in `rupa-engine` or `rupa-mobile-core`.

---

## 🗝️ Public API

### `struct App`
- `App::new(name)`: Sets the OS-level application name.
- `.root(component)`: Defines the entry point of the VNode tree.
- `.run()`: Bootstraps the Desktop GUI environment (Winit/WGPU).
- `.run_tui()`: Bootstraps the Terminal environment (Crossterm).
- `.run_mobile()`: Bootstraps the Mobile environment (Android/iOS).

---

## 🔄 Interaction Flow
- **App Builder -> Platform Runner:** Triggers the platform-specific execution loop.
- **App Builder -> Plugin System:** Invokes plugin build hooks.
- **App Builder -> Theme Engine:** Initializes the `rupa-ui` Theme Engine using `rupa-vnode` data models.
