# Module: Asset Cache (`crates/rupa-assets/src/cache.rs`) 💾

This module implements the in-memory storage for already loaded and decoded assets.

---

## 🧩 `struct Cache`
A thread-safe, LRU (Least Recently Used) cache for binary resources.

- **Deduplication**: Ensures the same asset is not loaded multiple times.
- **Memory Pressure**: Automatically evicts unused assets based on configurable memory limits.
