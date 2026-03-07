# Module: Plugin System (`crates/rupa-engine/src/plugin.rs`) 🔌

The Plugin System is the extensibility gateway of the Rupa Framework. It allows developers to inject custom logic, global state, or theme presets into the framework's bootstrap process.

---

## 🏗️ Core Responsibilities

1.  **Extensibility:** Provides a standardized way to add features to the `App` without modifying the core codebase.
2.  **Registration Management:** Maintains a registry of active plugins and ensures they are built in the correct order.
3.  **Bootstrap Hook:** Executes plugin logic during the application bootstrap phase.

---

## 🗝️ Key API Elements

### `trait Plugin`
The contract for any extension:
- **`name()`**: Returns a unique name for identification.
- **`build(app)`**: Grants mutable access to the `App` instance for registration.

### `struct PluginRegistry`
- **`add(plugin)`**: Enqueues a new plugin for initialization.
- **`build_all(app)`**: Triggers the build sequence for all registered plugins.

---

## 🔄 Interaction Flow

- **Plugins -> App Orchestrator**: Plugins inject logic during the `App` creation phase (e.g., registering new global signals).
- **Plugins -> Styling Ecosystem**: Often used to register custom Themes or DNA Visual tokens.
- **Plugins -> Platform Runners**: Can be used to inject platform-specific initialization code.
