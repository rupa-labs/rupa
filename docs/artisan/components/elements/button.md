# Interactive Buttons 🔘

The `Button` is the primary interactive material in the Rupa Framework. It is designed to handle user intents across all platforms while maintaining high aesthetic standards.

---

## 🏗️ Core Variants

Rupa provides several semantic variants out of the box to communicate visual intent clearly to the user.

- **`Variant::Primary`**: The main action. Usually filled with the theme's primary color.
- **`Variant::Secondary`**: An alternative action. Usually outlined or with a subtle background.
- **`Variant::Danger`**: Destructive actions (Delete, Remove). Usually red.
- **`Variant::Success`**: Positive completion (Confirm, Finish). Usually green.

```rust
Button::new("Commit Changes").variant(Variant::Primary)
Button::new("Cancel").variant(Variant::Secondary)
```

---

## 🎨 Artisan Styling

Buttons fully support the **Artisan Scaling** system. Use `Scale` classes to ensure your buttons feel balanced within your layout.

```rust
Button::new("Small Action")
    .p(Scale::Xs)
    .rounded(Scale::Sm)
```

### Chaining Modifiers
You can chain multiple modifiers to achieve your exact vision:
```rust
Button::new("Aesthetic Button")
    .bg(Color::Zinc800)
    .px(Scale::Lg)
    .py(Scale::Sm)
    .font_bold()
```

---

## 🖱️ Handling Intents

Instead of listening for raw mouse clicks, Rupa buttons listen for **Intents**. The `.on_click()` method triggers when the user interacts with the button using a Pointer, Touch, or the Enter key.

```rust
let count = Signal::new(0);

Button::new("Increment")
    .on_click(move |_| count.update(|v| *v += 1))
```

---

## 🌳 Technical Manifestation

When rendered, the `Button` produces a `VNode::Element` with a `"button"` tag. This DNA is then consumed by the **Physical Adapter**:
- **Desktop**: Renders a GPU mesh with hover/active state shaders.
- **Terminal**: Renders an ANSI-colored box with Unicode text.
- **Web**: Renders a native `<button>` element for best accessibility.
