# `rupa` 🎨

**The Universal Facade.** This is the primary entry point for the Rupa Framework. It is a convenience crate that bundles all **Atoms** (Tier 1) and **Composites** (Tier 2) into a single, cohesive API.

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Universal Entry Point)
- **Purpose**: Provides the `rupa::prelude::*` to ensure developers have immediate access to all tools without manually managing 30+ internal dependencies.

## 🚀 Usage

Instead of importing individual Atoms, just use the Facade:

```rust
use rupa::prelude::*;

fn main() {
    let count = Signal::new(0);
    // ... start building
}
```
