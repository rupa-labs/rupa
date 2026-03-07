# Dynamic Component Attributes

Rupa Framework allows you to attach arbitrary metadata to any semantic component using the `Attributes` system. This is useful for storing custom data, integrating with external scripts, or defining platform-specific properties.

## 📐 The `attr()` Method

Every standard Rupa Framework component provides an `.attr(key, value)` method.

```rust
use rupa::elements::Button;

let btn = Button::new("Click Me")
    .attr("data-tracking-id", "hero-banner-01")
    .attr("custom-state", "ready");
```

---

## 🏗 Why Separate Attributes from Style?

Rupa Framework follows a strict **Separation of Concerns (SOC)**:
- **`Style`**: Defines *how* a component looks (visuals, layout, motion).
- **`Attributes`**: Defines *what* a component is or holds (metadata, data-attributes, unique identifiers).

This separation ensures that your styling logic remains clean and performant, while attributes provide the necessary hooks for complex application logic.

---

## 🗝 Accessing Attributes

When creating custom components, you can access the attributes map directly.

```rust
impl Component for MyWidget {
    fn render(&self) {
        if let Some(val) = self.attributes.get("data-tracking-id") {
            log::debug!("Found tracking ID: {}", val);
        }
    }
}
```

## 🚀 Practical Uses
- **Testing**: Add `data-testid` for automated UI tests.
- **Analytics**: Store tracking tokens directly on interactive elements.
- **Wasm Interop**: Pass specific keys that your JavaScript bridge can read from the WASM memory or DOM.
