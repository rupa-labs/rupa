# Module: Storage Backends (`crates/rupa-store/src/backends/`) 🔌

This module contains platform-specific implementations of the `Store` trait.

---

## 🏁 Implementations
- **`fs.rs`**: Standard File System I/O for Desktop.
- **`sqlite.rs`**: Relational database storage for Desktop and Mobile.
- **`web.rs`**: Browser `localStorage` and `sessionStorage` integration.
- **`idb.rs`**: High-capacity `IndexedDB` integration for WASM.
