# Module: Context Consumer (`crates/rupa-context/src/consumer.rs`) 📤

This module provides the logic for retrieving injected data.

---

## 🏗️ Logic
- **Nearest Ancestor**: Traverses up the VNode tree to find the closest `Provider` of the requested type.
- **Type Safety**: Uses Rust's type system to ensure that the retrieved context matches the expected type.
