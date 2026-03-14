# `rupa-server` 🌍

**The Backend Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to render Rupa applications into static HTML strings, enabling Server-Side Rendering (SSR) and Static Site Generation (SSG).

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Translates VNode trees directly into HTML syntax, bypassing any physical layout calculations or display hardware.

## 🛠️ Infrastructure Backends
- **Routing**: `axum` integration (optional)
- **Serialization**: Custom HTML builder optimized for speed.

## 🚀 Usage

```rust
use rupa_server::HtmlRenderer;

fn handler() -> String {
    let app = MyRootComponent::new();
    HtmlRenderer::render_to_string(&app)
}
```
