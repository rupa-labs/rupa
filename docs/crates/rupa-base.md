# `rupa-base` 🧱

**The Foundation of Rupa.** This crate provides the foundational **Atoms** and standard data structures used across every tier of the framework.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Strict data validation for colors and spatial types prevents out-of-bounds errors.
    - **Sustain (S2)**: Standardized `Display` and `Debug` implementations for all types.
    - **Scalable (S3)**: Copy-on-write optimizations and lightweight bit-packed IDs.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Color`** | High-precision color math. | OKLCH support, linear RGBA, Hex parsing. |
| **`Vec2`** | 2D Spatial calculations. | Operator overloads, normalization, distance math. |
| **`Rect`** | Axis-aligned bounding boxes. | Collision testing, intersection, expansion. |
| **`Id`** | Global unique identifiers. | Atomic 64-bit generation, semantic display (`rupa-:1`). |
| **`Error`** | System-wide error model. | Diagnostic-friendly with context tracking. |

## 🚀 Usage

```rust
use rupa_base::{Color, Vec2, Rect, Id};

// 1. Perceptually uniform colors
let accent = Color::oklch(0.7, 0.15, 120.0, 1.0);

// 2. Vector math with operator overloads
let pos = Vec2::new(10.0, 10.0) + Vec2::new(5.0, 0.0);
let dir = pos.normalize();

// 3. Bounding box logic
let bounds = Rect::new(0.0, 0.0, 100.0, 100.0);
assert!(bounds.contains(pos));

// 4. Unique IDs for reconciliation
let my_id = Id::next();
```

## 🧪 Testing & Reliability
- **Precision**: Verified color conversion math (OKLCH to Linear RGB) with high-tolerance tests.
- **Display**: Standardized `rupa-X` string format for easier debugging in the terminal and logs.
- **Serializability**: All base types implement `serde` for seamless system-wide communication.
