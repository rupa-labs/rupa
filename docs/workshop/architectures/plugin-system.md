# Plugin System Architecture 🧩

The **Plugin System** is the primary mechanism for extending the Rupa Framework. It follows a modular "Plug-and-Play" model, allowing Artisans to inject custom logic, themes, and sub-systems into the agnostic Kernel.

---

## 1. The Core Contract

The `Plugin` trait is the gateway for extensions. It provides a semantic interface for interacting with the `App` instance during its initialization.

- **`name()`**: Returns a unique identifier for the plugin (e.g., `"AuthPlugin"`, `"WGPURenderer"`).
- **`build()`**: The primary entry point. It receives a mutable reference to the `App`, allowing the plugin to:
    - Register **Action Handlers** on the Bus.
    - Inject **Global State** or Signals.
    - Configure **Theme Defaults**.
    - Initialize **Platform Runners** or Renderers.

---

## 2. Plugin Hierarchy

Following the **3-Tier Architecture**, plugins are typically categorized based on their scope:

### A. Material Plugins (Tier 1)
Inject low-level primitives or utilities.
*Example*: A `MathPlugin` that provides specialized reactive calculation primitives.

### B. Master's Craft Plugins (Tier 2)
Add high-level systems or logic to the Kernel.
*Example*: A `RouterPlugin` that adds navigation logic to the application.

### C. Showroom Plugins (Tier 3)
Connect the Kernel to the physical world. These are the most common plugins.
*Example*: A `DesktopRendererPlugin` that initializes WGPU and windowing systems.

---

## 3. The Registration Pipeline

1. **Instantiation**: The Artisan creates instances of the desired plugins.
2. **Registration**: Plugins are added to the `PluginRegistry` during the `App::new()` phase.
3. **Orchestration**: The `App` calls `build()` on every registered plugin sequentially.
4. **Activation**: Once all plugins are built, the `App` enters its execution loop.

---

## 4. Why Use Plugins?

1. **Zero-Knot Coupling**: The Kernel (`rupa-engine`) doesn't need to know that `WGPU` exists. It just knows it has plugins that might provide a renderer.
2. **Modular Configuration**: You can easily swap a `MockAuthPlugin` for a `FirebaseAuthPlugin` just by changing the plugin registration.
3. **Community Extensibility**: Third-party artisans can build and share their own plugins as independent crates.
