# Text & Typography 🔡

The `Text` component is the semantic material for displaying strings. It bridges the gap between raw reactive data and the platform's Typography Engine.

---

## 🎨 Advanced Typography

Rupa provides a rich, fluent API for styling text without leaving your component logic. Every style applied to a `Text` component is platform-agnostic.

### Heading Presets
Quickly apply standardized heading styles:
```rust
Text::new("Main Title").h1()
Text::new("Sub Section").h3()
```

### Semantic Variants
Apply colors based on the current theme's semantic palette:
```rust
Text::new("Action successful").success()
Text::new("Critical failure").error()
Text::new("Optional info").muted()
```

---

## 🗝️ Artisan API

| Method | Description |
| :--- | :--- |
| **`.bold()`** / **`.thin()`** | Adjust font weight using semantic classes. |
| **`.italic()`** | Apply italic slant. |
| **`.mono()`** | Use the theme's monospaced font family. |
| **`.size(f32)`** | Set a specific font size (Absolute). |
| **`.center()`** / **`.right()`** | Control text alignment within the container. |
| **`.truncate()`** | Automatically add an ellipsis (`...`) on overflow. |

---

## 🧬 Reactive Content

Because the `Text` component consumes a `Signal<String>`, it is inherently reactive. You can pass a `Signal` or a `Memo` to the constructor to create a living label:

```rust
let name = Signal::new("Artisan");

// This text updates automatically when 'name' changes
Text::new(Memo::new(move || format!("Hello, {}!", name.get())))
```

---

## 🌳 Technical Manifestation

When rendered, the `Text` component produces a `VNode::Element` with a `"span"` tag. This element carries the typographic DNA (Size, Color, Weight) which the **Physical Adapter** then uses to perform text shaping and rendering.
