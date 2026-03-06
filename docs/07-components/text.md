# Module: Text (`text.rs`) 🔡

The `Text` component is the atomic semantic unit for displaying strings within the UI. It bridges the gap between raw data and the Typography Engine (L2).

---

## 🏗️ Components

### `struct Text`
Handles the rendering of shaped text.
- **Logic:** Manages the UTF-8 content string and basic alignment metadata.
- **View:** Interacts with the `TextMeasurer` during layout and the `TextRenderer` during the paint phase.

---

## 🗝️ Semantic API

- `.style(text_color(Color))`: Standard way to change text aesthetics via modifiers.

## 💻 Usage Example

```rust
Text::new("Hello World")
    .style(text_color(Color::Zinc(100)))
```
