# Module: Plugin System (`plugin.rs`) 🔌

The Plugin system is the primary architectural vehicle for extending Rupa Framework without modifying the core framework.

---

## 🧠 Internal Anatomy

### 1. The Build Hook
Plugins implement the `Plugin` trait, which requires a `build()` method. This method is executed during the **Application Bootstrap (L8)**, granting the plugin mutable access to the `App` instance.

### 2. Dependency Injection
Plugins are typically used to:
- Register custom theme presets.
- Initialize global state.
- Attach platform-specific hooks.

---

## 🗝️ API Anatomy

- `trait Plugin`: The contract for all extensions.
- `struct PluginRegistry`: The internal container managed by `App`.

## 💻 Implementation Example

```rust
struct MyThemePlugin;

impl Plugin for MyThemePlugin {
    fn name(&self) -> &str { "CustomTheme" }
    fn build(&self, _app: &mut App) {
        Theme::update(|t| t.borders.radius = 4.0);
    }
}
```
