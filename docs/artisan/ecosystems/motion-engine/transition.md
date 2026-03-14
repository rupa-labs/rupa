# Module: Transitions (`crates/rupa-motion/src/transition.rs`) ✨

This module defines deterministic animations for VNode properties.

---

## ✨ `struct Transition`
Declarative configuration for property changes.

- **Properties**: Interpolates `Color`, `Vec2` (Position/Size), and `f32` (Opacity/Scale).
- **Easing**: Supports standard curves (Linear, EaseIn, EaseOut, etc.).
- **VNode Integration**: Attached via the `Motion` attribute in the `render()` method.
