# Section 🏛️

A semantic container for major UI regions.

---

## 🏗️ Purpose
`Section` helps organize your UI into meaningful blocks (e.g., "Hero Section", "Features Grid"). In TUI, it can be used to define logical screen areas.

---

## 🗝️ API
| Method | Description |
| :--- | :--- |
| **`.child(comp)`** | Adds content to the section. |

---

## 🚀 Example
```rust
Section::new()
    .child(Text::new("Features").h2())
    .child(FeaturesGrid::new())
```
