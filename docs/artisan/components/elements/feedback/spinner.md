# Spinner 🌀

An animated indicator for indeterminate loading states.

---

## 🏗️ Purpose
Use `Spinner` when you need to inform the user that a process is active but the exact completion time is unknown (e.g., fetching data from an API).

---

## 🚀 Example
```rust
Show::new(is_loading_signal)
    .child(Spinner::new())
```
