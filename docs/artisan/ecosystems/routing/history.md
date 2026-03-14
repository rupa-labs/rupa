# Module: History Abstraction (`crates/rupa-router/src/history.rs`) 🕰️

This module provides a unified interface for the application's navigation history.

---

## 🏗️ Trait: `History`
The bridge between the application's route state and the OS navigation.

- **Native Implementation**: Stores navigation history in a memory stack.
- **Web Implementation**: Bridges to the browser's `History API` (PushState/ReplaceState).
