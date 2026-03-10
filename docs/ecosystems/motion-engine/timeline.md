# Module: Animation Timeline (`crates/rupa-motion/src/timeline.rs`) ⏳

This module orchestrates complex sequences of animations across multiple components.

---

## ⏳ `struct Timeline`
A controller for playing, pausing, and scrubbing through a sequence of `Transitions`.

- **Sequence**: Orchestrates "First A, then B" animations.
- **Parallel**: Runs multiple animations simultaneously with synchronized starts.
- **Events**: Provides hooks for `on_start`, `on_complete`, and `on_loop`.
