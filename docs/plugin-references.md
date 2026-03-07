# Plugin & Extension Reference 🔌

Rupa Framework is designed to be extensible. Through the Plugin System, you can inject new logic, manage global state, or intercept system events before they reach the UI tree.

---

## 🏗️ The `Plugin` Trait
To create a plugin, implement the `Plugin` trait.

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn build(&self, app: &mut App);
}
```

- **`name()`**: A unique identifier for the plugin.
- **`build()`**: Called during the `App` bootstrap phase. This is where you register listeners or modify themes.

---

## 👂 Global Event Listeners
Plugins can monitor all application traffic without being part of the visual tree.

```rust
app.add_event_listener(|event| {
    if let InputEvent::Key { key, .. } = event {
        println!("Global key press detected: {:?}", key);
    }
});
```

Useful for:
- **Analytics**: Tracking user interactions.
- **Global Hotkeys**: Handling shortcuts like `Ctrl+S` anywhere in the app.
- **Logging**: Debugging input routing.

---

## 🎨 Theme Extensions
Plugins can inject entire theme presets or modify existing semantic colors.

```rust
impl Plugin for MyThemePlugin {
    fn build(&self, app: &mut App) {
        Theme::update(|t| {
            t.colors.insert("brand-gold".into(), Color::Oklch(0.8, 0.2, 80.0, 1.0));
        });
    }
}
```

---

## 🔄 Registration Lifecycle
1. **Instantiation**: `App::new()` creates the registry.
2. **Registration**: `.plugin(MyPlugin::new())` adds the plugin to the list.
3. **Bootstrap**: When `.run()` is called, the app iterates through all registered plugins and executes their `build()` methods.
4. **Active Phase**: Plugins remain active as global listeners or state managers throughout the app lifecycle.
