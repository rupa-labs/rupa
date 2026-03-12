# `rupa-motion` ✨

**The Soul of Rupa.** This crate provides physics-based and duration-based animation **Atoms**. It is the engine behind fluid, "alive" user interfaces.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Decoupled tick-based orchestration prevents frame-timing leakage.
    - **Sustain (S2)**: Abstract `Animatable` trait allows any type to be animated without changing the engine.
    - **Scalable (S3)**: Global timeline orchestrator prevents redundant update loops.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Spring<T>`** | Physics-based motion. | Mass, Stiffness, Damping, and Precision control. |
| **`Transition<T>`** | Duration-based tweening. | Pluggable Easing curves, Duration support. |
| **`Easing`** | Interpolation curves. | Cubic, Quint, Elastic, Bounce, etc. |
| **`Timeline`** | Global frame orchestrator. | Agnostic tick delivery for cross-platform support. |

## 🚀 Usage

```rust
use rupa_motion::{Spring, Transition, Easing};

// 1. Spring Animation (Natural physics)
let pos = Spring::new(0.0);
pos.set_target(100.0); // Animate toward target with default spring config

// 2. Styled Transitions (Duration based)
let opacity = Transition::new(0.0)
    .with_duration(500)
    .with_easing(Easing::CubicInOut);

opacity.set_target(1.0);

// 3. Reacting to motion
Effect::new({
    let pos = pos.clone();
    move || {
        println!("Current visual position: {}", pos.get());
    }
});
```

## 🧪 Testing & Reliability
- **Agnostic Tick**: The motion engine does not depend on system time or OS timers; it can be manually "ticked" in tests for deterministic verification.
- **Convergence**: Spring solvers are verified to converge accurately to targets within defined precision limits.
- **Zero-Dependency**: No dependency on WGPU or physical display drivers.
