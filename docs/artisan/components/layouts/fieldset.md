# Fieldset 🏷️

A specialized container that renders a border with a text label (legend) at the top.

---

## 🏗️ Purpose
`Fieldset` is essential for grouping related form inputs or creating "Panels" in TUI applications that need a clear visual boundary and title.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.new(label)`** | Initializes with a legend text. |
| **`.label_align(pos)`**| Aligns the legend (Left, Center, Right). |

---

## 🚀 Example
```rust
Fieldset::new("User Credentials")
    .child(Input::new("Username"))
    .child(Input::new("Password"))
```
