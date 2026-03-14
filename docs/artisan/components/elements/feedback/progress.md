# Progress 📊

A visual bar showing the completion status of a task or process.

---

## 🏗️ Purpose
`Progress` provides feedback for ongoing background operations. It supports both determinate (known %) and indeterminate (loading) states.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.value(signal)`** | Binds the progress percentage (0.0 to 1.0). |

---

## 🚀 Example
```rust
Progress::new(upload_percent_signal)
```
