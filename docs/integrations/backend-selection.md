# Backend Selection (GUI vs TUI) 🔄

Rupa Framework is designed to be highly modular. By changing a single configuration, you can switch your entire application from a high-performance GUI to a lightweight Terminal interface.

---

## 📦 Compilation-Time Selection (Cargo Features)

The most efficient way to select a backend is via Cargo features. This ensures that the unused Platform Integration code (e.g., WGPU for TUI) is not compiled into your final binary.

```toml
# In your Cargo.toml

# For a Native GUI App
rupa = { version = "0.1", features = ["gui"] }

# For a Terminal TUI App
rupa = { version = "0.1", features = ["tui"] }
```

---

## ⚙️ Runtime Handshake

If both features are enabled, Rupa Framework defaults to GUI but allows for a runtime handshake during the bootstrap phase:

```rust
fn main() {
    let app = App::new("My Artisan App");

    if std::env::args().any(|arg| arg == "--tui") {
        app.run_tui(); // Forces the TUI Platform Integration & Renderer
    } else {
        app.run(); // Defaults to GUI Platform Integration (WGPU/Winit)
    }
}
```

---

## 🤝 Feature Parity

Regardless of the backend selected at Layer 1:
- **State management** (Signals) remains identical.
- **Layouts** (Flexbox/Grid) are calculated by Taffy in both modes.
- **Component Logic** is shared 100%.

Only the **Rendering (L2)** and **Hardware Abstraction (L1)** change their implementation to suit the target environment.
