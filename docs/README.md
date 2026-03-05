# Rupaui Documentation Index

Welcome to the **Rupaui** documentation. Rupaui is a modern, cross-platform UI framework built with Rust, designed for artisans who value semantic structure and utility-first flexibility.

## 📚 Component Reference

1. [**Window**](./window.md) - Physical viewport and GPU surface.
2. [**Section**](./section.md) - Top-level semantic regions.
3. [**Div**](./div.md) - Generic grouping and containers.
4. [**Text**](./text.md) - Typography and content.

## 🎨 Styling & Design

1. [**Modular Styling**](./modular-styling.md) - **NEW!** Functional & atomic styling.
2. [**Styling & Colors**](./styling.md) - General Style object overview.
3. [**Layout, Flex & Grid**](./layout.md) - Positioning and structure.
4. [**Spacing & Sizing**](./spacing-sizing.md) - Dimensions and box model.
5. [**Typography**](./typography.md) - Deep dive into text utilities.
6. [**Background & Border**](./background-border.md) - Visual styling.
7. [**Visual Effects**](./effects.md) - Shadows, blending, and masking.
8. [**Filters & Backdrop**](./filters.md) - Image processing.
9. [**Motion & Transforms**](./motion-transform.md) - Animations and 3D geometry.
10. [**Tables**](./tables.md) - Structured data layout.
11. [**Interactivity & SVG**](./interactivity-svg.md) - Input and vector graphics.
12. [**Vector Math**](./vector-math.md) - High-precision calculations.
13. [**State Management**](./state-management.md) - Reactive UI with Signals.
14. [**Theming**](./theme.md) - Global design tokens.
15. [**Core Philosophy**](./philosophy.md) - "Utility-First, Semantic-Support".
16. [**WebAssembly & Cross-Platform**](./platforms.md) - Deployment guides.

## 🚀 Quick Start (Modular API)

```rust
use rupaui::prelude::*;
use rupaui::utils::{p, bg, rounded};

fn main() {
    let app = App::new();
    
    let box_element = Div::new()
        .style((
            p(20.0),
            bg(Color::Indigo(600)),
            rounded(12.0)
        ))
        .text("Modern Modular Styling!");
}
```
