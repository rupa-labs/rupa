# Persistent Data System: Overview 💾

The **`rupa-store`** system acts as a bridge between short-term reactive state (`rupa-signals`) and long-term hardware storage. In the Rupa meta-framework, persistence is not just I/O; it is a native part of the **Reactive Graph**.

---

## 1. Philosophy: "Storage as a Signal"

In Rupa, data stored on disk or in a database is treated as a **Persistent Signal**.
- **Auto-Sync**: Any mutation to the Signal automatically triggers a `Write` operation to the storage backend.
- **Auto-Load**: Upon application startup, the Signal is initialized with the last known value from storage.
- **Agnostic**: Application logic remains unaware of whether data is stored in SQLite, LocalStorage, or a flat file.

---

## 2. Module Structure (1:1 Mapping)

| Module | Responsibility | Description |
| :--- | :--- | :--- |
| `store.rs` | **Trait Store** | Universal abstraction for Get/Set/Delete operations. |
| `signal.rs` | **PersistentSignal** | Reactive wrapper connecting a Signal to a Store. |
| `backends/` | **Implementations** | Platform-specific drivers (SQLite, IndexedDB, FS). |
| `encryption.rs`| **Security (S1)** | Optional layer for encrypting data before it hits the disk. |

---

## 3. Artisan Usage Example

```rust
// Define a signal that automatically syncs to LocalStorage with key "user_theme"
let theme = PersistentSignal::new("user_theme", ColorMode::Dark);

// UI updates, AND the database/file updates atomically.
theme.set(ColorMode::Light);
```
