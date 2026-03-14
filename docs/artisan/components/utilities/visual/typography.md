# Typography Utilities 🔡

Refine the appearance of text elements.

---

## 🏗️ Usage
These utilities can be applied to `Text` components or any container to set inherited text styles.

### API
| Method | Description | Example |
| :--- | :--- | :--- |
| **`.font_size(val)`** | Sets text size in pixels. | `.font_size(24.0)` |
| **`.font_bold()`** | Sets font weight to Bold. | `.font_bold()` |
| **`.text_color(c)`** | Sets the text color. | `.text_color(Color::White)` |

---

## 🚀 Example
```rust
VStack::new()
    .font_bold()
    .text_color(Color::Blue)
    .child(Text::new("All children are bold and blue"))
```
