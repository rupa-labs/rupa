# Motion & Transforms

Rupaui provides professional-grade tools for temporal animations and spatial 3D transforms, allowing you to create fluid, interactive artisan interfaces.

## 🌀 Spatial Transforms

Transforms allow you to manipulate an element's geometry without affecting the surrounding layout.

- `.translate(x, y, z)`: Move the element along 3D axes.
- `.rotate(x, y, z)`: Rotate the element (in degrees).
- `.scale(x, y, z)`: Resize the element.
- `.skew(x, y)`: Distort the element.
- `.transform_origin(x, y, z)`: Set the pivot point for transforms.
- `.perspective(f32)`: Set the 3D depth.
- `.transform_style(TransformStyle)`: Choose between `Flat` or `Preserve3d`.
- `.backface(BackfaceVisibility)`: Control if the back of an element is visible when rotated.

```rust
Style::new()
    .rotate(0.0, 45.0, 0.0)
    .perspective(1000.0)
    .transform_origin(0.5, 0.5, 0.0)
```

---

## ⏳ Motion (Transitions & Animations)

Control how properties change over time.

### Transitions
Transitions smooth out property changes (e.g., hover effects).

- `.transition(property, duration_ms, timing_function)`

```rust
Style::new()
    .transition("background-color", 300.0, TimingFunction::EaseInOut)
```

### Keyframe Animations
Trigger named keyframe sequences.

- `.animate(name, duration_ms, timing_function)`

---

## 🗝 Key Types
- **TimingFunction**: `Ease`, `Linear`, `EaseIn`, `EaseOut`, `EaseInOut`, `StepStart`, `StepEnd`, `CubicBezier(f32, f32, f32, f32)`.
- **TransformStyle**: `Flat`, `Preserve3d`.
