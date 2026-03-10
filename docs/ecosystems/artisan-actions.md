# Artisan Actions (The Command Bus) 🚌

Artisan Actions are modular units of logic that can be executed from anywhere within the Rupa ecosystem—be it a UI component, a background service, or the `rupa` CLI.

---

## 🏗️ The Action Architecture

Artisan Actions follow the **Command Bus** pattern, separating the "Intent" (Action) from the "Execution" (Handler).

### 1. Define the Action (Data)
An action is a simple serializable struct representing the request.
```rust
use serde::{Serialize, Deserialize};
use rupa::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct CreateComponent {
    pub name: String,
}

impl Action for CreateComponent {}
```

### 2. Define the Handler (Logic)
The handler contains the actual "Artisan logic".
```rust
pub struct CreateComponentHandler;

impl ActionHandler<CreateComponent> for CreateComponentHandler {
    fn handle(&self, action: CreateComponent) -> Result<(), Error> {
        println!("🎨 Etching component {} into the workshop...", action.name);
        Ok(())
    }
}
```

---

## 🎨 Registration (Provide & Free)

You register Actions in your `App` during initialization. This makes them discoverable by the `rupa run` command.

```rust
fn main() {
    let app = App::new("My App")
        .action::<CreateComponent>("make:component", CreateComponentHandler);
    
    // Check if an action was requested via CLI
    if app.handle_cli_actions() {
        return; // Exit if action was handled
    }

    app.run();
}
```

---

## 🚀 Execution (The "Take Anywhere" Principle)

### From the CLI
Use the `rupa run` command to dispatch an action. You can optionally provide a JSON payload.
```bash
rupa run make:component --payload '{"name": "MyButton"}'
```

### From UI Code
You can trigger the exact same logic from any component:
```rust
Button::new("Create")
    .on_click(|_| {
        // Access the global app bus to dispatch
        App::dispatch("make:component", CreateComponent { name: "MyButton".into() });
    })
```

---

## 💡 Why this is superior?
- **Headless Mode**: Execute backend logic or migrations without booting the GPU/TUI.
- **Unified Logic**: One handler for both CLI commands and UI interactions.
- **Testability**: Easily unit-test handlers by passing mock actions.
