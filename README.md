# Rupaui 🎨

**Rupaui** is a modern, high-performance cross-platform UI framework for Rust. It is engineered for artisans who demand the perfect balance between **Semantic Structure** and **Utility-First Flexibility**.

Inspired by the ergonomics of TailwindCSS and the reliability of Bootstrap, Rupaui provides a type-safe, reactive engine built directly on top of **WGPU** and **Taffy**.

## 🌟 Key Features

- **Utility-First, Semantic-Support**: Compose complex visual identities using a functional API while maintaining a clean, meaningful component hierarchy.
- **Signal-Based Reactivity**: Fine-grained state management using `Signal` and `Memo` for zero-overhead UI updates.
- **DNA Visual (Theme Engine)**: Centralized design tokens that dictate the "factory settings" (borders, typography, variants) for all components.
- **Artisan Palette**: A mathematically verified color system using **OKLCH** for perceptual uniformity and consistent aesthetics.
- **Wasm & Native**: First-class support for both WebAssembly (browser) and Native Desktop (GPU-accelerated).
- **Extensible Architecture**: Modular kerangka that allows users to freely create their own utilities and semantic elements.

## 🚀 Quick Start (Modern Modular API)

```rust
use rupaui::prelude::*;
use rupaui::utils::{p, bg, rounded, hover, active};

fn main() {
    // 1. Define reactive state
    let count = Signal::new(0);
    
    // 2. Compose UI with Semantic Components and Modular Styling
    let my_button = Button::new("Click Me")
        .variant(Variant::Primary)
        .on_click({
            let count = count.clone();
            move || count.update(|v| *v += 1)
        })
        .style((
            p(16.0),
            rounded(8.0),
            hover(bg(Color::Indigo(500))),
            active(scale(0.95, 0.95, 1.0))
        ));
}
```

## 🏗 Component Hierarchy

Rupaui follows a strict semantic flow:
1. **`Window`**: The physical OS viewport or browser canvas.
2. **`Section`**: High-level structural regions (Sidebar, Header, etc.).
3. **`Layout`**: `Container`, `Row`, and `Col` for grid-based positioning.
4. **`Elements`**: `Button`, `Input`, `Card`, `Modal`, `Text`, `SVG`, and more.

## 🛠 Tech Stack

- **Graphics**: WGPU (WebGPU/Vulkan/Metal/DX12)
- **Layout**: Taffy (Flexbox & CSS Grid)
- **Windowing**: Winit v0.30
- **Reactivity**: Custom Signal/Memo Engine
- **Typography**: Glyph-brush (Planned integration)

## 📖 Documentation

Detailed guides and API references are available in the [`/docs`](./docs/README.md) directory:
- [Core Philosophy](./docs/core/philosophy.md)
- [State Management](./docs/core/state-management.md)
- [Modular Styling](./docs/styling/modular-styling.md)
- [Theming (DNA Visual)](./docs/styling/theme.md)

---
Built with ❤️ by [Reas Vyn](https://github.com/reasnov).
