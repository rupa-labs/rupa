# Rupaui 🎨

**Rupaui** is a modern, high-performance cross-platform UI framework for Rust. It is engineered for artisans who demand the perfect balance between **Semantic Structure**, **Fine-Grained Reactivity**, and **Hardware-Accelerated Performance**.

Rupaui is built on a modular 9-layer architecture, allowing the same UI logic to power both 4K graphical applications (via WGPU) and lightweight terminal interfaces (via TUI).

---

## 🌟 Key Features

- **Utility-First, Semantic-Support:** Atomic styling with a functional API, backed by a clean, meaningful component hierarchy.
- **Logic & View Architecture:** Strict separation between UI state (Logic) and rendering infrastructure (View).
- **Universal Multi-Backend:** Native support for **GUI (WGPU/Winit)** and **TUI (Crossterm)** with 100% logic reuse.
- **Signal-Based Reactivity:** Zero-overhead UI updates using a precise Signal/Memo engine.
- **DNA Visual System:** Perceptually uniform colors via **OKLCH** and a 10-step unified scale for spacial integrity.
- **High-Performance Rendering:** 2D Batching, SDF geometry, and hardware-accelerated typography.

---

## 🚀 Quick Start

Add Rupaui to your `Cargo.toml`:

```toml
[dependencies]
rupaui = "0.1"
```

Create a reactive counter in a few lines:

```rust
use rupaui::prelude::*;

fn main() {
    let count = Signal::new(0);

    App::new("Artisan Counter")
        .root(
            VStack::new()
                .style((flex(), justify_center(), items_center(), h_full()))
                .child(Box::new(Text::new(Memo::new({
                    let count = count.clone();
                    move || format!("Count: {}", count.get())
                }))))
                .child(Box::new(Button::new("Increment")
                    .variant(Variant::Primary)
                    .on_click(move |_| count.update(|v| *v += 1))
                ))
        )
        .run();
}
```

---

## 🏗️ The 9-Layer Blueprint

Rupaui is designed with total transparency. Explore our architecture from hardware to ecosystem:

1.  [**HAL Layer**](./docs/01-hal/platform-orchestrator.md) - OS & Hardware abstraction.
2.  [**Rendering Engine**](./docs/02-rendering/renderer-interface.md) - WGPU and TUI painters.
3.  [**Geometric Scene**](./docs/03-layout/scene-core.md) - Tree resolution and Taffy integration.
4.  [**Reactivity**](./docs/04-reactivity/signals.md) - Signals and Change Propagation.
5.  [**Architecture**](./docs/05-architecture/logic-and-view.md) - Component design standards.
6.  [**Primitives**](./docs/06-primitives/primitive-design.md) - Atomic building blocks.
7.  [**Components**](./docs/07-components/component-design.md) - The Artisan Library.
8.  [**Composition**](./docs/08-composition/app-bootstrap.md) - Application assembly.
9.  [**Ecosystem**](./docs/09-ecosystem/styling-api.md) - DNA Visual & Design System.

---

## 🤝 Contributing

We welcome contributions from everyone! Please see our [CONTRIBUTING.md](./CONTRIBUTING.md) and [Engineering Standards](./docs/engineering-standards.md) to get started.

## ⚖️ License

Rupaui is dual-licensed under the [MIT License](./LICENSE). 

Built with ❤️ by [Reas Vyn](https://github.com/reasnov).
