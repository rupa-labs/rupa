# Getting Started with Rupa Framework 🚀

Welcome, Artisan! This guide will walk you through setting up your environment and crafting your first multi-platform application using the Rupa Framework. 

Rupa is designed for those who value the **integrity of the craft**. Whether you are building a high-performance desktop suite, a sleek terminal utility, or a modern web experience, Rupa provides the high-quality materials you need to succeed.

---

## 🛠️ Prerequisites

Before you enter the workshop, ensure your system has the following tools:

1.  **Rust Toolchain**: Rupa requires the latest stable version of Rust. Install it via [rustup.rs](https://rustup.rs/).
2.  **System Dependencies** (Required for hardware-accelerated Desktop rendering):
    *   **Linux**: `libwayland-dev`, `libx11-dev`, `pkg-config`, `libfontconfig1-dev`.
    *   **macOS**: Standard Xcode Command Line Tools.
    *   **Windows**: Visual Studio Build Tools with C++ support.

---

## 🎨 Method 1: The Rupa Wizard (Recommended)

The fastest way to scaffold a new project is using our TUI-powered CLI tool. It automates the setup of the **Atoms & Composites** structure for you.

### 1. Install the CLI (Coming Soon)
```bash
# cargo install rupa-cli
```

### 2. Craft a New Project
```bash
rupa create
```
Choose **"Zero Bloat (Pure Logic)"** for a minimal starting point, or **"Native Power"** for a full-featured desktop template.
 Explore all available [**Project Templates**](./templates/README.md).

Follow the aesthetic on-screen prompts to choose your project name, template (Desktop, TUI, or Full-stack), and initial theme.

---

## 📝 Method 2: Manual Setup

If you prefer to hand-pick every piece of your material, follow these steps:

### 1. Initialize your Workspace
```bash
mkdir my-artisan-app && cd my-artisan-app
cargo init
```

### 2. Add the Materials
Add Rupa to your `Cargo.toml`. During the early development phase, we recommend using the Git repository:

```toml
[dependencies]
# For a standard Desktop application
rupa = { git = "https://github.com/rupa-labs/rupa", features = ["desktop"] }
```

### 3. Write Your First Masterpiece
Replace the contents of `src/main.rs` with this basic reactive counter:

```rust
use rupa::prelude::*;

fn main() {
    // 1. Define your reactive state
    let count = Signal::new(0);

    // 2. Build your app using aesthetic Tiers
    App::new("Artisan Counter")
        .root(
            VStack::new()
                .p(Scale::S4)      // Aesthetic padding
                .items_center()
                .gap(Scale::S2)    // Standardized spacing
                .child(Box::new(Text::new(Memo::new({
                    let count = count.clone();
                    move || format!("Current Value: {}", count.get())
                })).font_bold()))
                .child(Box::new(Button::new("Increment")
                    .variant(Variant::Primary)
                    .on_click(move |_| count.update(|v| *v += 1))
                ))
        )
        .run();
}
```

---

## 🏗️ Core Concepts for Artisans

To master Rupa, you must understand three foundational principles:

### 1. Fine-Grained Reactivity 🧬
Rupa doesn't "re-render everything." It uses **Signals** to create a living heartbeat for your UI. When a value changes, only the specific text node or style property that depends on it is updated. This results in extreme performance and predictable behavior.
- *Learn more in [**Reactivities & Events**](../reactivities/README.md).*

### 2. The Universal DNA (VNodes) 🌳
Your components produce **VNodes** (Virtual Nodes). A VNode is a technical description of your intent. It doesn't know about pixels or ANSI characters yet. This decoupling allows your logic to be 100% platform-agnostic and easily testable.
- *Learn more in [**Component Library**](../components/README.md).*

### 3. Showrooms & Adapters 🏪
The **Showroom** is where your creation is displayed. By changing the **Adapter**, you can run the exact same logic on the GPU (**Desktop Adapter**), in the terminal (**Terminal Adapter**), or in a browser (**Web Adapter**).
- *Explore available targets in [**Template References**](./templates/README.md).*

---

## 🚀 Next Scales

Now that you have your first app running, it's time to dive deeper:

1.  **Explore Components**: See the full list of available materials in the [**Component References**](../components/README.md).
2.  **Add Interactivity**: Learn how to handle complex user intents in the [**Event References**](../reactivities/event-references.md).
3.  **Design your Aesthetic**: Master the [**Theme System**](./templates/README.md) and OKLCH color spaces.
4.  **Go Full-Stack**: Learn how to build content-driven sites using the [**Markdown Engine**](../ecosystems/README.md).

---

*Crafting software is a journey of continuous improvement. We are excited to see what you will build with the Rupa Framework.*
