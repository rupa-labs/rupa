# `rupa-motion` ✨

**The Animation Ports.** Physics-based and duration-based motion engine.

## 🛠️ Key Features

- **`Spring`**: Realistic physics-based interpolation.
- **`Transition`**: Duration-based tweening with Easing curves.
- **`Easing`**: Standard curves (Cubic, Expo, Elastic, etc.).
- **`GLOBAL_TIMELINE`**: Agnostic tick-based orchestrator.

## 🚀 Usage

```rust
use rupa_motion::{Spring, Transition, Easing};
use std::time::Duration;

// 1. Spring Animation
let scale = Spring::new(1.0);
scale.set_target(1.5); // Smoothly animates via physics

// 2. Duration Transition
let opacity = Transition::new(0.0);
opacity.set_target(1.0); // Animates over 300ms (default) with CubicInOut
```
