# Getting Started with Rupa Framework 🚀

Welcome, Artisan! This guide will walk you through setting up your environment and crafting your first multi-platform application using the Rupa Framework. 

Rupa is designed for those who value the **integrity of the craft**. Whether you are building a high-performance desktop suite, a sleek terminal utility, or a modern web experience, Rupa provides the high-quality materials you need to succeed.

---

## 🛠️ Prerequisites

Before you enter the workshop, ensure your system has the following tools:

1.  **Rust Toolchain**: Rupa requires the latest stable version of Rust. Install it via [rustup.rs](https://rustup.rs/).
2.  **System Dependencies**: Required for hardware-accelerated rendering. See [**Installation Guide**](./installation.md).

---

## 🎨 Method 1: The Rupa Wizard (Recommended)

The fastest way to scaffold a new project is using our TUI-powered CLI tool.

### 1. Install the CLI (Coming Soon)
```bash
# cargo install rupa-cli
```

### 2. Choose Your Path
When you run `rupa create`, you will select one of the **12 Artisan Paths**. Each path is a pre-configured template tailored for a specific intent:

| Category | Artisan Path | Command Flag |
| :--- | :--- | :--- |
| **Core Showrooms** | [Native Power (Desktop)](./templates/native-power.md) | `--template native` |
| | [Web Excellence (Web)](./templates/web-excellence.md) | `--template web` |
| | [Terminal Arts (TUI)](./templates/terminal-arts.md) | `--template terminal` |
| | [Mobile Mobility (Mobile)](./templates/mobile-mobility.md) | `--template mobile` |
| **Logic & Server** | [Zero Bloat (Pure Logic)](./templates/zero-bloat.md) | `--template pure` |
| | [Server Authority (Server)](./templates/server-authority.md) | `--template server` |
| | [Pure Logic (Headless)](./templates/headless-logic.md) | `--template headless` |
| **Advanced Fusions** | [Fullstack Fusion](./templates/fullstack-fusion.md) | `--template fullstack` |
| | [Hybrid Interop](./templates/hybrid-interop.md) | `--template hybrid` |
| **Artisan Material** | [The Master's App](./templates/library-build.md) | `--template app` |
| | [Plugin Creation](./templates/plugin-creation.md) | `--template plugin` |
| | [Library Build (Handcraft)](./templates/library-build.md) | `--template library` |

---

## 📝 Method 2: Manual Setup

If you prefer to hand-pick every piece of your material, follow ini:

### 1. Initialize your Workspace
```bash
mkdir my-artisan-app && cd my-artisan-app
cargo init
```

### 2. Add the Materials
Add Rupa to your `Cargo.toml`:
```toml
[dependencies]
rupa = { git = "https://github.com/rupa-labs/rupa", features = ["desktop"] }
```

### 3. Write Your First Masterpiece
```rust
use rupa::prelude::*;

fn main() {
    App::new("Artisan App")
        .root(VStack::new().p(Scale::Md).child(Text::new("Hello Rupa")))
        .run();
}
```

---

## 🏗️ Core Concepts for Artisans

To master Rupa, you must understand three foundational pillars:

1.  **Fine-Grained Reactivity 🧬**: Use [**Signals**](../reactivities/signals.md) to drive your UI.
2.  **The Universal DNA 🌳**: Your components produce [**VNodes**](../reactivities/README.md) (Virtual Nodes).
3.  **Hardware Adapters 🏪**: The same logic runs anywhere via [**Adapters**](../adapters/README.md).

---

*Crafting software is a journey of continuous improvement. We are excited to see what you will build.*
