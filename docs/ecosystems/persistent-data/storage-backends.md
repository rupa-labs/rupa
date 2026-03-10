# Module: Storage Backends (`rupa-store::backends`) 🔌

This document defines how the Rupa Framework interacts with various physical storage media across different platforms.

---

## 🏗️ Trait `Store` (Agnostic Bridge)

All backends must implement this trait to be compatible with `PersistentSignal`.

```rust
pub trait Store: Send + Sync {
    fn read(&self, key: &str) -> Result<Option<Vec<u8>>, Error>;
    fn write(&self, key: &str, value: &[u8]) -> Result<(), Error>;
    fn delete(&self, key: &str) -> Result<(), Error>;
}
```

---

## 🧩 Per-Platform Implementations

### 1. `FileSystemStore` (Native Desktop)
- **Module**: `backends::fs`
- **Mechanism**: Saves data in the user's config directory (e.g., `~/.config/rupa-app/`). Each key maps to a file.
- **Pros**: Extremely fast for small data, zero external dependencies.

### 2. `SqliteStore` (Native Desktop/Mobile)
- **Module**: `backends::sqlite`
- **Mechanism**: Uses a relational database file.
- **Pros**: Supports complex queries and transactional integrity (ACID).

### 3. `WebStorageStore` (WASM)
- **Module**: `backends::web`
- **Mechanism**: Uses `localStorage` or `sessionStorage` APIs.
- **Pros**: Standard across all browsers, synchronous and easy to use.

### 4. `IndexedDbStore` (WASM)
- **Module**: `backends::idb`
- **Mechanism**: Asynchronous in-browser database.
- **Pros**: Large capacity (GigaBytes), ideal for cached assets.

---

## 🛠️ Modul Detail: `lib.rs`

The `rupa-store` crate will export helper functions for automatic backend initialization:

```rust
// Automatic initialization based on target platform
let store = rupa_store::auto_init(); 
```
