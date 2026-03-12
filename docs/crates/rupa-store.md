# `rupa-store` 💾

**The Persistence Port.** Reactive signals that automatically synchronize state with permanent storage.

## 🛠️ Key Features

- **`PersistentSignal<T>`**: Automatically saves state on change (set/update).
- **`Store`**: Trait defining backend operations (read, write, delete).
- **`FileSystemStore`**: Standard backend for Desktop file storage.
- **`Effect-based Syncing`**: Zero-manual-work persistence.

## 🚀 Usage

```rust
use rupa_store::{PersistentSignal, FileSystemStore};
use std::sync::Arc;

// 1. Setup Backend
let fs = Arc::new(FileSystemStore::new("my-app"));

// 2. Create Persistent State
let settings = PersistentSignal::new("settings", AppSettings::default(), fs);

// 3. Just use it like a normal signal
settings.update(|s| s.dark_mode = true); // Automatically saved to disk!
```
