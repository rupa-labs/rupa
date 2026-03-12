# `rupa-assets` 📦

**The Resource Loading Port.** Reactive management of external assets like images and fonts.

## 🛠️ Key Features

- **`Loader`**: Port for fetching raw bytes (FS, Network, Embedded).
- **`Manager`**: Central orchestrator with multi-adapter support.
- **`Cache`**: Thread-safe reactive storage for loaded assets.
- **`LoadTask`**: Integration with background queues for non-blocking loading.

## 🚀 Usage

```rust
use rupa_assets::Manager;

// 1. Register a loader (Adapter)
manager.register_loader(Box::new(NetworkLoader::new())).await;

// 2. Load an asset
let bytes = manager.load_raw("https://example.com/logo.png").await?;

// 3. Queue-based loading
manager.load_queued("assets/font.ttf")?;
```
