# Rupaui Plugin System

The Plugin System allows you to modularize your application logic and extend Rupaui's core capabilities. This is inspired by the modular patterns found in modern Rust engines like Bevy.

## 📐 Defining a Plugin

To create a plugin, implement the `Plugin` trait.

```rust
use rupaui::prelude::*;
use rupaui::utils::{Theme, Variant, Color};

pub struct MyArtisanPlugin;

impl Plugin for MyArtisanPlugin {
    fn name(&self) -> &str { "MyArtisanPlugin" }

    fn build(&self, _app: &mut App) {
        // 1. Register custom theme defaults
        Theme::update(|t| {
            t.variants.insert(Variant::Primary, Color::Rose(500));
        });
        
        log::info!("Artisan Plugin initialized!");
    }
}
```

---

## 🏗 Registering Plugins

Plugins are registered during the `App` initialization phase.

```rust
fn main() {
    App::new("Rupa Editor")
        .add_plugin(MyArtisanPlugin)
        .run();
}
```

---

## 🚀 Key Benefits
- **Modularity**: Keep your feature logic separated into independent crates or modules.
- **Reusability**: Share common plugins (like an Analytics plugin or a specialized UI Kit) across multiple Rupaui projects.
- **Bootstrapping**: Plugins provide a dedicated phase to configure global states, themes, and design tokens before the main UI loop starts.

## 🗝 Practical Uses
- **Theme Plugins**: A plugin that only handles Dark/Light mode logic.
- **Service Plugins**: Integrating API clients or external database connections.
- **A11y Plugins**: Automatically enforcing specific accessibility rules across the entire app.
