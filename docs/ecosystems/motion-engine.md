# Module: Animation & Motion Engine (`crates/rupa-motion`) ✨

The `rupa-motion` crate provides a high-performance, non-blocking animation system for the Rupa Framework. 

---

## 1. Core Philosophy: VNode Interpolation
Instead of relying on heavy CSS transitions or manual `request_animation_frame` loops, `rupa-motion` calculates intermediate VNode values (interpolations) and sends them directly to the `Renderer` as efficient `Update` patches.

## 2. Technical Architecture

### 2.1 Reactive Integration
Motion in Rupa is driven by **Reactive Interpolators**. These are specialized `Signal` variants that don't jump instantly to a new value, but instead emit a series of intermediate values over time based on physics or duration rules.

- **`Animate<T>`**: A wrapper that intercepts a `Signal<T>` change and applies a `Transition`.
- **`Spring<T>`**: A physics-based interpolator that handles stiffness, damping, and mass.

### 2.2 The Motion Loop
To maintain performance and keep Tier 1 (Signals) decoupled from platform-specific timers:
1.  **Ticker**: A global, platform-agnostic ticker (provided by Tier 2/3 runners) triggers delta-time updates.
2.  **State Update**: Motion signals calculate their new values.
3.  **VNode Patch**: The change triggers a reactive `Effect`, which generates a `Patch::Update` for the renderer.

### 2.3 Declarative API
Artisans can define motion directly within the style or component structure:

```rust
Button::new("Hover Me")
    .style(hover(
        Style::new()
            .scale(1.1)
            .transition(Transition::ease_in_out(200))
    ))
```

---

## 3. Implementation Plan (TDD)

The following primitives will be implemented in `crates/rupa-motion/src/`:
1.  **`spring.rs`**: Core physics engine for spring-based interpolation.
2.  **`transition.rs`**: Easing functions (Linear, EaseIn, Cubic, etc.) and duration-based logic.
3.  **`timeline.rs`**: Coordination of multiple concurrent animations.
