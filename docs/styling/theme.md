# Theming & Global Configuration

The `Theme` module is the **Authoritative Configuration Engine** for Rupaui. It defines the "Visual DNA" of your application by setting factory defaults for all components.

## 🧬 Structural Defaults (DNA Visual)

You can define how components are born by configuring global border and typography standards. This ensures consistency without repeated manual styling.

```rust
Theme::update(|t| {
    t.borders.width = 2.0;      // All components inherit 2px borders
    t.borders.radius = 8.0;     // All components inherit 8px rounding
    t.typography.base_size = 18.0; 
});
```

### 🗝 `apply_defaults(&mut Style)`
Internally, Rupaui uses this method to sync a component's style with the global DNA. You can also use it in custom components to maintain consistency.

---

## 🏗 Themed Constructors

Every standard Rupaui component is **Theme-Aware** from the moment it is created.

```rust
// This button automatically inherits the global border, 
// radius, and font size defined in the active Theme.
let btn = Button::new("Themed Button");
```

---

## 🔤 Font Stacks & Fallbacks

Rupaui supports semantic font stacks, ensuring that your typography remains consistent even if a specific font fails to load.

```rust
Theme::update(|t| {
    t.typography.font_stacks.insert("sans".into(), vec![
        "Inter".into(), 
        "Roboto".into(), 
        "sans-serif".into()
    ]);
});
```

---

## 🏷 Semantic Variants

Variants (`Primary`, `Success`, `Danger`, etc.) are mapped to specific colors. Changing a variant in the theme updates all components globally.

```rust
Theme::update(|t| {
    t.variants.insert(Variant::Primary, Color::Rose(600));
});
```

## 🚀 Dynamic Switching
Theme data is reactive. If you update the `Theme` at runtime (e.g., switching to Dark Mode), all theme-aware components will reflect the changes on the next paint pass.
