# `rupa-engine` ⚙️

**The Agnostic Kernel.** Manages the global application lifecycle and plugin orchestration.

## 🛠️ Key Features

- **`App`**: The main orchestrator for Rupa applications.
- **`Plugin`**: System for extending the framework with custom logic.
- **`ElementTree`**: Utilities for logical tree manipulation.
- **Agnostic State**: Handles events and redraws in a platform-independent way.

## 🚀 Usage

```rust
use rupa_engine::App;

// 1. Create a new App instance
let app = App::new("My Artisan App")
    .root(MyRootComponent::new())
    .plugin(MyCustomPlugin::new());

// 2. Run the application (using a specific Showroom)
app.run_terminal(); // For TUI
```
