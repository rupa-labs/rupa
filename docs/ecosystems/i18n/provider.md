# Module: I18n Provider (`crates/rupa-i18n/src/provider.rs`) 🌍

This module defines the context provider for internationalization.

---

## 🏗️ `struct I18nProvider`
A component wrapper that provides translation state to its entire sub-tree.

- **Reactive Language**: Holds a `Signal<Locale>` that triggers re-renders when the language is changed.
- **Context Injection**: Uses the framework's context system to allow any component to call `t!`.
