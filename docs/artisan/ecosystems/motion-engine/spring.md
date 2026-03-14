# Module: Spring Physics (`crates/rupa-motion/src/spring.rs`) 🎡

This module implements momentum-based animations using spring physics formulas.

---

## 🎡 `struct Spring<T>`
A reactive signal that interpolates its value based on physical properties (stiffness, damping, mass).

- **Natural Motion**: Unlike linear timers, springs feel "alive" and respond naturally to target interruptions.
- **Frame Sync**: Updates are synced with the framework's global frame clock.
