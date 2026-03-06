# DNA Visual (Theme Engine)

The theme system in Rupaui ensures aesthetic consistency throughout the application.

## Color Space: OKLCH
Rupaui uses the **OKLCH** color space for better perceptual uniformity. This allows us to adjust lightness without distorting chroma (saturation).

```rust
Color::Oklch(0.6, 0.12, 250.0, 1.0) // Lightness, Chroma, Hue, Alpha
```

## Unified Scale
The framework implements a 10-step scale for dimensions, spacing, and typography:
`Xs`, `Sm`, `Md` (Default), `Lg`, `Xl`, `Xl2`, `Xl3`, `Xl4`, `Xl5`, `Xl6`.

Applications:
- **Button:** `.size(Scale::Lg)`
- **Spacing:** `p_scale(Scale::Md)`
- **Layout Gap:** `gap_scale(Scale::Sm)`
- **Breakpoints:** `Breakpoint::Xl2`

## Theme Defaults
Default themes can be accessed and updated globally:

```rust
Theme::update(|t| {
    t.borders.radius = 8.0;
    t.colors.insert("primary".into(), Color::Indigo(500));
});
```
