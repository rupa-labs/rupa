# Filters & Backdrop Utilities

Rupaui provides a powerful set of image processing utilities that can be applied to elements or their backgrounds (backdrops). These are highly optimized for GPU-accelerated rendering.

## 🌈 Element Filters

Filters are applied to the element itself, including its content.

- `.blur(f32)`: Apply a Gaussian blur.
- `.brightness(f32)`: Adjust brightness (1.0 is default).
- `.contrast(f32)`: Adjust contrast.
- `.drop_shadow(x, y, blur, color)`: Apply a hardware-accelerated drop shadow.
- `.grayscale(f32)`: Convert to grayscale (0.0 to 1.0).
- `.hue_rotate(f32)`: Rotate hue in degrees.
- `.invert(f32)`: Invert colors.
- `.saturate(f32)`: Adjust color saturation.
- `.sepia(f32)`: Apply a sepia effect.

```rust
Style::new()
    .blur(4.0)
    .grayscale(0.5)
    .brightness(1.2)
```

---

## 🌫 Backdrop Utilities

Backdrop utilities apply filters to the area **behind** the element. This is perfect for creating "glassmorphism" or frosted glass effects.

- `.backdrop_blur(f32)`
- `.backdrop_brightness(f32)`
- `.backdrop_contrast(f32)`
- `.backdrop_grayscale(f32)`
- `.backdrop_hue_rotate(f32)`
- `.backdrop_invert(f32)`
- `.backdrop_opacity(f32)`
- `.backdrop_saturate(f32)`
- `.backdrop_sepia(f32)`

```rust
Style::new()
    .bg(Color::White(0.2)) // Semi-transparent background
    .backdrop_blur(10.0)   // Frosted glass effect
    .backdrop_saturate(1.5)
```

## 🗝 Implementation Note
Filters are stored in a `FilterStack`, allowing multiple filters to be applied in sequence. During the painting phase, these are typically converted into a single shader pass for maximum performance on WebAssembly and Native platforms.
