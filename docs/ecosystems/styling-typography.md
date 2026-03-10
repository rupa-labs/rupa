# Styling: Typography & Flow 🔡

The Rupa Framework provides a rich, type-safe system for managing typography via the `rupa-vnode` crate. This system allows you to define everything from basic font properties to advanced text decorations and flow control.

---

## 🔡 Font Properties (Atoms)
The foundational building blocks for text appearance.

- `.font(String)`: Set the font family.
- `.font_size(f32)`: Set the font size in pixels.
- `.font_bold()`: Shorthand for Bold weight.
- `.weight(FontWeight)`: Set font thickness (Thin, Regular, Bold, etc.).
- `.italic()`: Set font style to italic.

```rust
Text::new("Artisan Typography")
    .style((font_size(18.0), font_bold(), italic()))
```

---

## 🎨 Color & Alignment

- `.color(Color)`: Set text color using the Artisan Palette or Hex.
- `.align(TextAlign)`: Align text (Left, Center, Right, Justify).
- `.transform(TextTransform)`: Change text case (Uppercase, Lowercase, Capitalize).

```rust
Style::new()
    .color(Color::Indigo(500))
    .align(TextAlign::Center)
    .transform(TextTransform::Uppercase)
```

---

## ✨ Decorations

- `.underline()`: Shorthand for simple underline.
- `.decoration(line, style, color)`: Set complex text decorations.
- `.underline_offset(f32)`: Set distance between text and underline.

---

## 🌊 Text Flow & Spacing

- `.letter_spacing(f32)`: Set horizontal spacing between characters.
- `.line_height(f32)`: Set vertical spacing between lines.
- `.line_clamp(u32)`: Limit text to a specific number of lines.
- `.text_overflow(TextOverflow)`: Handle text that exceeds its box (Clip, Ellipsis).
- `.text_wrap(TextWrap)`: Control line wrapping (Wrap, NoWrap, Balance, Pretty).
- `.indent(f32)`: Set indentation for the first line.
- `.white_space(WhiteSpace)`: Control handling of whitespace and line breaks.

---

## 🏗 Advanced Layout

- `.v_align(VerticalAlign)`: Align text vertically (Top, Middle, Bottom, etc.).
- `.word_break(WordBreak)`: Control where words should break.
- `.hyphens(Hyphens)`: Control hyphenation behavior.
- `.content(String)`: For generated content or dynamic replacements.

```rust
Style::new()
    .line_height(1.5)
    .letter_spacing(0.5)
    .text_overflow(TextOverflow::Ellipsis)
    .line_clamp(2)
```
