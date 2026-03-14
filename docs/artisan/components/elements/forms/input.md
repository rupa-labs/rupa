# Input ⌨️

The primary material for capturing single-line text data.

---

## 🏗️ Purpose
`Input` acts as a reactive bridge between user typing and a `Signal<String>`.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.value(signal)`** | Binds to a reactive string signal. |
| **`.on_submit(f)`** | Triggers when the user presses Enter. |

---

## 🚀 Example
```rust
let name = Signal::new(String::new());
Input::new("Username").value(name)
```
