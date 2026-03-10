# Module: Dictionary Management (`crates/rupa-i18n/src/dictionary.rs`) 📚

This module handles the loading and indexing of translated strings.

---

## 🧩 `struct Dictionary`
A thread-safe registry of key-value pairs for translations.

- **Formats**: Supports JSON and Fluent (`.ftl`) files.
- **Interpolation**: Handles dynamic arguments within translation strings (e.g., "Hello, {name}").
