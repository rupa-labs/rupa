# `rupa-headless` 👻

**The Invisible Showroom.** This crate provides the **Adapters & Infrastructure** (Tier 3) to run Rupa applications without any physical display.

---

## 🏛️ Architectural Role
- **Tier**: Tier 3 (Showrooms)
- **Identity**: The Finished Showroom (Adapters & Infrastructure)
- **Responsibility**: Executes the application lifecycle and reactive state machine purely in memory. Ideal for automated testing, background workers, or scraping engines.

## 🛠️ Key Features
- **Zero Graphics Overhead**: No `wgpu` or `taffy` layout engine initialized.
- **Event Simulation**: Allows injecting simulated events into the engine to test application logic programmatically.

## 🚀 Usage

```rust
use rupa_headless::prelude::*;

fn run_worker() {
    let app = App::new("Headless Worker").root(MyWorkerComponent);
    // Runs in memory, reacting to network or time events without a UI
    app.run_headless();
}
```
