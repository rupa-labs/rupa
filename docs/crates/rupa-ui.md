# `rupa-ui` 🎨

**The Semantic UI Toolkit.** High-level, platform-agnostic components and a fluent styling API.

## 🛠️ Key Features

- **Semantic Elements**: `Button`, `Input`, `Card`, `Navbar`, etc.
- **Fluent Styling API**: Modifier-based styling (e.g., `.p(20).bg_primary().font_bold()`).
- **Responsive Primitives**: `VStack`, `HStack`, `Flex`, `Grid`.
- **Control Flow**: Reactive `Show` and `ForEach` components.

## 🚀 Usage

```rust
use rupa_ui::prelude::*;

pub fn my_view() -> impl Component {
    VStack::new()
        .p(24)
        .gap(12)
        .child(Text::new("Welcome Artisan").font_bold().font_size(32.0))
        .child(Button::new("Get Started").bg_primary())
}
```
