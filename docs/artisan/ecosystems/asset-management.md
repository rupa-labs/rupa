# Module: Asset Management System (`crates/rupa-assets`) 📦

The `rupa-assets` crate provides a unified pipeline for loading, decoding, and caching binary resources (Images, Fonts, Shaders, Audio).

---

## 1. Core Philosophy: Asynchronous Loading
Assets are large. Loading them must not block the main thread. This crate provides a reactive wrapper where an asset starts as a "Loading" state and resolves to a memory buffer when ready.

## 2. Platform Implementations
- **Native**: Loads directly from `std::fs` or embedded binaries (`include_bytes!`).
- **Web**: Loads via async `fetch` requests.

## 3. Key Exports (1:1 Mapping)
- `AssetManager`: The global registry and cache.
- `ImageBuffer` / `FontBuffer`: Decoded binary representations ready for the GPU or DOM.
- `use_asset(path)`: A reactive hook to request an asset.
