# Universal Facade Architecture 🎨

The **Universal Facade** (the `rupa` crate) is the primary high-level entry point for Artisans.

---

## 1. Technical Purpose

It acts as a **Flat Prelude**, re-exporting the most commonly used Atoms, Composites, and Showroom utilities into a single, ergonomic namespace.

---

## 2. Artisan Experience

Instead of importing from 30+ independent crates, the Artisan simply uses:
```rust
use rupa::prelude::*;
```
This ensures a low barrier to entry while maintaining the modularity of the underlying system.
