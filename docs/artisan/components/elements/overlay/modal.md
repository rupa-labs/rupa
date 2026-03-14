# Modal 🖼️

A focused dialog box that captures the user's attention.

---

## 🏗️ Purpose
Modals are used for critical interactions that require the user to focus on a specific task or confirmation before continuing. They typically appear centered on the screen with a backdrop.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.child(comp)`** | Sets the content of the modal. |

---

## 🚀 Example
```rust
Modal::new()
    .child(Text::new("Confirm deletion?"))
    .child(Button::new("Yes").on_click(|_| delete_action()))
```
