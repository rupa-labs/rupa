# `rupa-assets` 📦

**The Resource Port.** This crate provides **Atoms** for reactive resource loading and caching. It manages external files like images, fonts, and data blobs in a non-blocking way.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Content-addressable caching prevents resource injection and collisions.
    - **Sustain (S2)**: `use_asset` hook provides a simple, reactive way to fetch data.
    - **Scalable (S3)**: Multi-threaded loader support handles large-scale asset pools efficiently.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`Manager`** | Asset Orchestrator. | Tracks loading progress, stats, and cache integrity. |
| **`Loader`** | The Asset Port. | Trait for fetching bytes from different sources. |
| **`Cache`** | In-Memory storage. | Thread-safe, reactive map of loaded resource bytes. |
| **`use_asset`** | Reactive Hook. | Returns a `Signal<Option<Vec<u8>>>` for async loading. |
| **`MockLoader`** | Testing Backend. | Returns pre-configured bytes for deterministic tests. |

## 🚀 Usage

```rust
use rupa_assets::{use_asset, Manager, MockLoader};

// 1. Register a loader (usually during app setup)
let manager = Manager::new().with_loader(Arc::new(MockLoader::new()));

// 2. Use the reactive hook in a component
let logo = use_asset("textures/logo.png");

Effect::new(move || {
    if let Some(bytes) = logo.get() {
        println!("Logo loaded: {} bytes", bytes.len());
    }
});
```

## 🧪 Testing & Reliability
- **Deterministic Loading**: `MockLoader` ensures that tests don't fail due to network or disk I/O issues.
- **Reactive Consistency**: Verified that UI components correctly re-render when an asset finishes loading.
- **Stats Tracking**: `Manager` tracks metrics like cache hits and loading times for observability.
