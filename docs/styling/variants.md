# Interaction Variants (Hover, Focus, etc.)

Rupaui supports state-driven styling similar to TailwindCSS, allowing you to define different visual properties for different interaction states using a fluent Rust API.

## 🖱 Supported Variants

- `.hover(Modifier)`: Applied when the mouse pointer is over the element.
- `.focus(Modifier)`: Applied when the element has keyboard focus.
- `.active(Modifier)`: Applied when the element is being pressed/clicked.
- `.disabled_style(Modifier)`: Applied when the component's `disabled` signal is true.

---

## 🚀 Basic Usage

You can pass any `StyleModifier` (atomic functions, tuples, or full Style objects) to a variant method.

```rust
use rupaui::utils::{bg, color, rounded, p, scale, translate};

Button::new("Interactive Artisan")
    .style((
        bg("zinc-800"),
        p(12.0),
        // Hover state
        hover(bg("indigo-600")),
        // Active/Pressed state (adds tactile feedback)
        active((
            bg("indigo-700"),
            scale(0.98, 0.98, 1.0),
            translate(0.0, 1.0, 0.0)
        )),
        // Focus state (Accessibility)
        focus(outline(2.0, BorderStyle::Solid, "white")),
        // Disabled state
        disabled_style((
            opacity(0.5),
            cursor(Cursor::NotAllowed)
        ))
    ));
```

---

## 🏗 How it Works

1.  **State Detection**: Rupaui components track internal states (Hovered, Focused, Pressed) and reactive states (Disabled).
2.  **Style Merging**: During the `Paint` phase, active variants are merged on top of the base style in this priority order: `Base` -> `Hover` -> `Focus` -> `Active` -> `Disabled`.
3.  **Efficiency**: Variants use `Option<Box<Style>>` to ensure zero memory overhead for components that don't use them.

## 💡 Best Practices
- Always provide a `.focus()` style for keyboard accessibility.
- Use `.active()` to provide immediate tactile feedback during a click.
- Combine variants with `.transition()` for professional, smooth animations.
