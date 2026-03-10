# Rupa Framework 🎨

**Rupa Framework** is a **modular meta-framework, cross-platform and multi-purpose**, engineered for Rust artisans. It is built as a highly decoupled ecosystem of **Atomic Materials & Composite Assembly Assemblies**, allowing you to use it as a complete high-level solution or as a collection of specialized standalone tools for any target.

---

## 🌟 The Artisan Spirit

Rupa Framework is designed for developers who demand the perfect balance between **Semantic Structure**, **Fine-Grained Reactivity**, and **Industry-Standard Modularity**.

- **Atomic Materials:** Independent low-level building blocks like `rupa-signals` (reactivity) and `rupa-vnode` (design DNA).
- **Composite Assembly Assemblies:** High-level systems that orchestrate multiple atomic materials, such as `rupa-ui` (components) and `rupa-engine` (native rendering).
- **Artisan Showrooms:** Ready-to-use specialized artisan showrooms like `rupa-desktop`, `rupa-web`, or `rupa-server`.

---

## 🏗️ The Showroom (Target-Driven Choice)

Rupa is engineered as an **"IKEA-style"** ecosystem. You decide the scale of your framework:

1.  **Native Power:** High-performance rendering via `rupa-desktop` or `rupa-tui`.
2.  **Web Excellence:** Modern DOM-based rendering with `rupa-server` (SSR) and `rupa-web` (SPA).
3.  **Headless Logic:** Just use `rupa-headless` for logic-only automation or background services.

For a full list of crates, see [**Crate References**](./docs/crate-references.md).

---

## 🚀 Quick Start (Showroom Mode)

Add Rupa to your `Cargo.toml`:

```toml
[dependencies]
# For Desktop (WGPU) applications
rupa = { version = "0.1", features = ["desktop"] }

# For Full-Stack Web (SSR + DOM) applications
rupa = { version = "0.1", features = ["web", "ssr"] }
```

Create a reactive counter that works across GPU, Terminal, and Web:

```rust
use rupa::prelude::*;

fn main() {
    let count = Signal::new(0);

    App::new("Artisan Counter")
        .root(
            VStack::new()
                .style((p(20.0), items_center()))
                .child(Box::new(Text::new(Memo::new({
                    let count = count.clone();
                    move || format!("Count: {}", count.get())
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
