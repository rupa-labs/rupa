# Module: Asset Manager (`crates/rupa-assets/src/manager.rs`) 📦

This module defines the central registry for all application resources.

---

## 🏗️ `struct AssetManager`
The unified interface for requesting resources like images, fonts, and shaders.

- **Reactive Hook**: Provides `use_asset` which returns a signal that resolves when the asset is ready.
- **Reference Counting**: Tracks how many components are using an asset to optimize memory release.
