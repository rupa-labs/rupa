# Path: Plugin Creation 🔌

The **Plugin Creation** path is for Artisans who want to extend the Rupa Framework itself. By crafting a plugin, you can inject new behaviors, global states, or hardware adapters into any Rupa application.

---

## 🏗️ What is a Rupa Plugin?

A plugin is a modular "extension cord" that plugs into the **App Orchestrator** during the bootstrap phase. It is the primary way to share complex, reusable functionality across the ecosystem.

- **Hook-Based**: Plugins interact with the `App` through the `build()` method.
- **System Injection**: You can register new **Action Handlers**, inject **Global Signals**, or configure **Theme Defaults**.
- **Showroom Fulfillment**: Most hardware adapters (like WGPU or Crossterm) are delivered to the Kernel through plugins.

---

## 🎯 Best For:

1.  **Hardware Adapters**: If you are building a new renderer for a specific device.
2.  **Shared Cross-cutting Logic**: Authentication kits, global logging, or telemetry providers.
3.  **Third-party Integrations**: Connecting Rupa to external databases, cloud services, or other Rust libraries.
4.  **Aesthetic Presets**: Creating reusable theme packs that can be easily "plugged in" to any project.

---

## 🚀 Quick Start

Initialize via CLI:
```bash
rupa create --template plugin
```

### Basic Plugin Blueprint
```rust
use rupa_engine::plugin::Plugin;
use rupa_engine::App;

pub struct MyArtisanPlugin;

impl Plugin for MyArtisanPlugin {
    fn name(&self) -> &str { "my-artisan-plugin" }

    fn build(&self, app: &mut App) {
        // Inject your craft here
        log::info!("Plugin '{}' is being assembled.", self.name());
    }
}
```

---

*Plugins are the connective tissue of the Rupa ecosystem. Craft yours with excellence.*
