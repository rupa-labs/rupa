# Module: Animation & Motion Engine (`crates/rupa-motion`) ✨

The `rupa-motion` crate provides a high-performance, non-blocking animation system for the Rupa Framework. 

---

## 1. Core Philosophy: VNode Interpolation
Instead of relying on heavy CSS transitions or manual `request_animation_frame` loops, `rupa-motion` calculates intermediate VNode values (interpolations) and sends them directly to the `Renderer` as efficient `Update` patches.

## 2. Key Concepts
- **Spring Physics**: Natural, momentum-based animations rather than linear easing.
- **Keyframes**: Deterministic point-to-point transitions for Styles (Color, Layout, Transform).

## 3. Key Exports (1:1 Mapping)
- `Spring<T>`: A reactive numeric signal that smoothly interpolates to its target value.
- `Transition`: Configuration for animation duration and easing curves.
- `Motion`: The VNode attribute that declares how an element enters, exits, or updates.
