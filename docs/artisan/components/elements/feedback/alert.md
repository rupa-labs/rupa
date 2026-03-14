# Alert ⚠️

Important messages or notification banners that require the user's attention.

---

## 🏗️ Purpose
`Alert` components are used to communicate status changes, warnings, or errors. They are usually pre-styled with high-visibility semantic colors.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.variant(v)`** | Sets the semantic color (`Danger`, `Warning`, etc). |

---

## 🚀 Example
```rust
Alert::new("Your session is about to expire.").variant(Variant::Warning)
```
