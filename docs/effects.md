# Visual Effects & Masking

Rupaui provides advanced visual compositing utilities, allowing for complex layer effects, shadows, and masking that are optimized for GPU rendering.

## 🌑 Shadows

- `.shadow(x, y, blur, color)`: Add a box shadow. Can be called multiple times for layered shadows.
- `.text_shadow(x, y, blur, color)`: Add a shadow specifically for text content.

```rust
Style::new()
    .shadow(0.0, 4.0, 10.0, Color::Black(0.3))
    .shadow(0.0, 10.0, 20.0, Color::Black(0.1))
```

---

## 🎭 Blending Modes

Blending modes control how an element's colors mix with the content behind it.

- `.blend(BlendMode)`: Set the `mix-blend-mode` for the element.
- `.bg_blend(BlendMode)`: Set the `background-blend-mode` for background layers.

### Supported Modes:
`Normal`, `Multiply`, `Screen`, `Overlay`, `Darken`, `Lighten`, `ColorDodge`, `ColorBurn`, `HardLight`, `SoftLight`, `Difference`, `Exclusion`, `Hue`, `Saturation`, `Color`, `Luminosity`.

---

## 🌫 Opacity & Z-Index

- `.opacity(f32)`: Set global element transparency (0.0 - 1.0).
- `.z(i32)`: Set the stacking order.

---

## 🎭 Masking

Masking allows you to hide parts of an element using images or gradients.

- `.mask_image(String)`: Set the mask source image.
- `.mask_size(BackgroundSize)`: Control mask scaling.
- `.mask_repeat(BackgroundRepeat)`: Control mask tiling.
- `.mask_mode(MaskMode)`: Choose between `Alpha` or `Luminance` masking.
- `.mask_composite(MaskComposite)`: Define how multiple mask layers combine (`Add`, `Subtract`, `Intersect`, `Exclude`).

```rust
Style::new()
    .mask_image("assets/masks/circle.png")
    .mask_mode(MaskMode::Alpha)
```

## 🗝 Key Types
- **BlendMode**: Standard compositing modes.
- **MaskMode**: `Alpha`, `Luminance`, `MatchSource`.
- **MaskComposite**: `Add`, `Subtract`, `Intersect`, `Exclude`.
