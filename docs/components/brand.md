# Module: Brand & Identity (`brand.rs`) 🏷️

This module provides the semantic landmark for application identity, typically used in Navbars or Footer sections.

---

## 🏗️ Components

### `struct Brand`
A compound component that combines a name and an optional logo.
- **Logic:** Manages the brand string and logo asset path.
- **View:** Provides a horizontal layout with specific brand-aware spacing.

---

## 🗝️ Semantic API

- `.logo(path)`: Attaches an image or vector logo.
- `.id(name)`: Sets the semantic identifier.

## 💻 Usage Example

```rust
Brand::new("Rupa Framework")
    .logo("assets/logo.png")
```
