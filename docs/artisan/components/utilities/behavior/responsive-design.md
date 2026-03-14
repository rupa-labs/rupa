# Responsive Design 📱

Conditional styling based on screen size using the 10-scale breakpoint system.

---

## 🏗️ Usage
Rupa Framework follows a **mobile-first** approach. Use these modifiers to adapt your layout for larger screens.

### Breakpoints
| Method | Min-Width | Context |
| :--- | :--- | :--- |
| **`.xs(mod)`** | 0px | Extra Small (Mobile) |
| **`.sm(mod)`** | 640px | Small (Large Mobile) |
| **`.md(mod)`** | 768px | Medium (Tablet) |
| **`.lg(mod)`** | 1024px | Large (Laptop) |
| **`.xl(mod)`** | 1280px | Extra Large (Desktop) |
| **`.xl2(mod)`** to **`.xl6(mod)`** | 1536px+ | Large Desktop / Ultra-wide |

---

## 🚀 Example
```rust
VStack::new()
    .flex_col() // Default
    .md(flex_row()) // Switch to row on tablets
    .lg(gap(Scale::Xl)) // Increase gap on laptops
```
