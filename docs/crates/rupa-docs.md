# `rupa-docs` 📚

**The Interactive Documentation.** This crate is an internal Tooling application built *with* Rupa, designed to showcase the framework's capabilities across all rendering targets.

---

## 🏛️ Architectural Role
- **Category**: Developer Tooling / Example Application
- **Identity**: The Living Blueprint
- **Responsibility**: Proves the "Write Once, Run Anywhere" philosophy by rendering the same documentation component tree in Desktop, Terminal, and Web showrooms.

## 🚀 Implementation Proof

```rust
use rupa::prelude::*;

/// The exact same component is rendered in all showrooms
pub fn docs_app() -> impl Component {
    VStack::new()
        .p(20.0)
        .child(Text::new("Rupa Framework").style(font_bold()))
        .child(Button::new("Read Docs").variant(Variant::Primary))
}
```
