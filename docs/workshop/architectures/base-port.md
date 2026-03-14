# Base Port Architecture 🧱

The **Base Port** provides the foundational, platform-agnostic invariants and standard types used by every other crate in the Rupa ecosystem.

---

## 1. Core Types

- **`Vec2`**: Standard 2D coordinate container.
- **`Color`**: Universal color model supporting OKLCH and RGBA.
- **`Id`**: Globally unique identifier generator.
- **`Error`**: Standardized framework-wide error handling.

---

## 2. Technical Context

- **Zero Dependencies**: This crate has zero internal framework dependencies, ensuring it remains at the bottom of the hierarchy.
- **Stable Contract**: Types defined here are intended to be immutable and stable across all platform versions.
