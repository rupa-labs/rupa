# `rupa-store` 📦

**The Persistence Port.** This crate provides **Atoms** for reactive state persistence, automatically synchronizing `Signal` data with permanent storage backends.

---

## 🏛️ Architectural Role
- **Tier**: Tier 1 (Atoms)
- **Identity**: The Materials & Tools (Ports & Invariants)
- **3S Compliance**: 
    - **Secure (S1)**: Support for encrypted backends and atomic write operations.
    - **Sustain (S2)**: `PersistentSignal` provides a zero-boilerplate API for state that survives reboots.
    - **Scalable (S3)**: Effect-based synchronization ensures only changed keys are written to disk.

## 🛠️ Key Primitives

| Primitive | Purpose | Features |
| :--- | :--- | :--- |
| **`PersistentSignal<T>`** | Synced reactive state. | Integrated with `rupa-signals`, automatic background saving. |
| **`Store`** | The Persistence Port. | Trait for Read/Write/Delete operations on bytes. |
| **`FileSystemStore`** | Desktop Backend. | Directory-based key-value storage. |
| **`MockStore`** | Testing Backend. | In-memory key-value map for TDD. |

## 🚀 Usage

```rust
use rupa_store::{PersistentSignal, MockStore, Store};
use std::sync::Arc;

// 1. Setup Backend (Mock for tests, FileSystem for prod)
let store: Arc<dyn Store> = Arc::new(MockStore::new());

// 2. Create Persistent State (Atoms)
let config = PersistentSignal::new("app_config", Config::default(), store);

// 3. Update state - saving is handled automatically by the reactive effect
config.update(|c| c.theme = "Dark".into());
```

## 🧪 Testing & Reliability
- **Mock Implementation**: Built-in `MockStore` allows for verified persistence logic without file system side-effects.
- **TDD Driven**: Synchronous and asynchronous persistence flows are verified via the `rupa-test` suite.
- **Serializability**: Leverages `serde_json` for cross-platform data compatibility.
