# Getting Started with Rupa Framework 🚀

Welcome, Artisan! This guide will walk you through setting up your environment and building your first multi-platform application with Rupa.

---

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed on your system:

1.  **Rust Toolchain**: Install via [rustup.rs](https://rustup.rs/).
2.  **System Dependencies** (for Native Desktop rendering):
    *   **Linux**: `libwayland-dev`, `libx11-dev`, `pkg-config`.
    *   **macOS/Windows**: Standard C++ build tools.

---

## 🎨 Method 1: The Rupa Wizard (Recommended)

The easiest way to start is using our TUI-powered project initializer.

### 1. Install the CLI
```bash
cargo install rupa-cli
```

### 2. Craft a New Project
```bash
rupa create
```
Follow the aesthetic on-screen prompts to choose your project name and template.

---

## 📝 Method 2: Manual Setup

If you prefer to understand every piece of your material, follow these steps:

### 1. Initialize Cargo
```bash
mkdir my-artisan-app && cd my-artisan-app
cargo init
```

### 2. Add Dependencies
Add the following to your `Cargo.toml`:

```toml
[dependencies]
rupa = { git = "https://github.com/rupa-labs/rupa", features = ["desktop"] }
```

### 3. Write Your First Component
Replace the contents of `src/main.rs`:

```rust
use rupa::prelude::*;

fn main() {
    App::new("Hello Rupa")
        .root(
            VStack::new()
                .p(40.0)
                .gap(20.0)
                .items_center()
                .child(Box::new(Text::new("Welcome to the Workshop").font_bold()))
                .child(Box::new(Button::new("Click Me").on_click(|_| println!("Artisan button clicked!"))))
        )
        .run();
}
```

---

## 🏗️ Core Concepts for Artisans

### 1. The Reactivity Model
Rupa uses **Fine-Grained Reactivity**. Instead of re-rendering the entire tree, only the specific VNode properties that depend on a `Signal` are updated.

### 2. The Tiered Architecture
- **Tier 1 (Atomic)**: Raw materials like `Signal`, `VNode`, and `Color`.
- **Tier 2 (Composite)**: Assemblies like `Button`, `VStack`, and `Input`.
- **Tier 3 (Showroom)**: Complete platform runners like `rupa-desktop`.

### 3. Platform Agnosticism
Your code describes **intent** via VNodes. The same `VStack` can be rendered as a GPU primitive, an HTML `div`, or a terminal block depending on which showroom you run it in.

---
## 🚀 Next Steps

### Generate a Static Site
If you have markdown files in `src/pages`, generate your HTML output:
```bash
rupa build
```

### Run Custom Actions
Execute internal application logic without the UI:
```bash
rupa run my:action
```

### Learn More
- Explore the [**Component References**](./component-references.md) to see available building blocks.
... (rest of the file) ...

- Learn about [**Motion & Animations**](./ecosystems/motion-engine.md) to make your UI feel alive.
- Check out the [**Engineering Standards**](./engineering-standards.md) to contribute back to the ecosystem.

Happy Crafting! 🎨
