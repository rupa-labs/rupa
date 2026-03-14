# Navbar 🧭

The standard header material for global application navigation.

---

## 🏗️ Purpose
`Navbar` typically stays at the top of the viewport and contains the logo, primary links, and global utilities like search or profile.

---

## 🚀 Example
```rust
Navbar::new()
    .child(Brand::new("Rupa"))
    .child(Spacer::new())
    .child(Button::new("Login"))
```
