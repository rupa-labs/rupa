# Module: Persistent Data System (`crates/rupa-store`) 💾

The `rupa-store` crate provides a universal, platform-agnostic interface for persistent storage. It bridges the gap between Rupa's reactive memory (`rupa-signals`) and long-term storage mechanisms.

---

## 1. Core Philosophy: Reactive Persistence
Data should not just be saved; it should be integrated into the Reactive Graph. If a database value changes, the UI should update automatically.

## 2. Platform Implementations (The Bridge)
- **Desktop/Mobile**: Binds to SQLite or local file system (`std::fs`).
- **Web (WASM)**: Binds to `IndexedDB` or `localStorage`.
- **Server (SSR)**: Binds to Redis or PostgreSQL connection pools.

## 3. Key Exports (1:1 Mapping)
- `Store`: The main trait for KV or Relational storage.
- `PersistentSignal<T>`: A special signal that automatically syncs its state to the `Store`.
