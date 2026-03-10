# Rupa Framework 🎨

**Rupa Framework** is a **modular meta-framework, cross-platform and multi-purpose**, engineered for Rust artisans. It is built as a highly decoupled ecosystem of **Atomic Materials & Composite Assembly Assemblies**, allowing you to use it as a complete high-level solution or as a collection of specialized standalone tools for any target.

---

## 🌟 The Artisan Spirit

Rupa Framework is designed for developers who demand the perfect balance between **Semantic Structure**, **Fine-Grained Reactivity**, and **Industry-Standard Modularity**.

- **Atomic Materials:** Independent low-level building blocks like `rupa-signals` (reactivity) and `rupa-vnode` (design DNA).
- **Composite Assembly Assemblies:** High-level systems that orchestrate multiple atomic materials, such as `rupa-ui` (components) and `rupa-engine` (native rendering).
- **Artisan Showrooms:** Ready-to-use specialized artisan showrooms like `rupa-desktop`, `rupa-web`, or `rupa-server`.

---

## 🏗️ Getting Started (The Artisan Way)

The fastest way to start crafting with Rupa is through our aesthetic installation wizard.

### 1. Install the Rupa CLI
Ensure you have [Rust](https://www.rust-lang.org/) installed, then run:

```bash
cargo install rupa-cli
```

### 2. Create Your Project
Launch the crafting wizard and follow the intuitive steps:

```bash
rupa create
```

For more detailed instructions, see the [**Getting Started Guide**](./docs/getting-started.md).

---

## 🚀 Manual Quick Start (Showroom Mode)

If you prefer to set up manually, add Rupa to your `Cargo.toml`:

```toml
[dependencies]
# For Desktop (WGPU) applications
rupa = { git = "https://github.com/rupa-labs/rupa", features = ["desktop"] }
```

Create a reactive counter that works across Desktop, Terminal, and Web:

```rust
use rupa::prelude::*;

fn main() {
    // 1. Define your reactive state
    let count = Signal::new(0);

    // 2. Build your app using aesthetic Tiers
    App::new("Artisan Counter")
        .root(
            VStack::new()
                .p(32.0)
                .items_center()
                .gap(16.0)
                .child(Box::new(Text::new(Memo::new({
                    let count = count.clone();
                    move || format!("Current Value: {}", count.get())
                }))))
                .child(Box::new(Button::new("Increment")
                    .on_click(move |_| count.update(|v| *v += 1))
                ))
        )
        .run();
}
```

---

## 🛠️ Project Structure

- `crates/rupa-*`: The core ecosystem (Atomic Materials & Composite Assembly Assemblies).
- `docs/`: In-depth architectural guides and standards.
- `examples/`: Multi-platform implementation demos.

Learn more about how we build crates in [**Crate Construction**](./docs/crate-construction.md).

---

## 🤝 Contributing

We welcome contributions from everyone! Please see our [**CONTRIBUTING.md**](./CONTRIBUTING.md) and [**Engineering Standards**](./docs/engineering-standards.md) to get started.

## ⚖️ License

Rupa Framework is dual-licensed under the MIT License.

Built with ❤️ by [Reas Vyn](https://github.com/reasnov).
