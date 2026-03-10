use std::sync::Arc;
use rupa_core::Component;
use rupa_core::action::{Action, ActionHandler};
use crate::platform::context::{PlatformCore, SharedPlatformCore};
...
    pub fn debug(self, enabled: bool) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.debug = enabled;
        }
        self
    }

    pub fn action<A: Action + 'static>(self, name: impl Into<String>, handler: impl ActionHandler<A> + 'static) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.action_handlers.insert(name.into(), Box::new(handler));
        }
        self
    }

    /// Processes any Artisan Actions requested via CLI flags.
    /// Returns true if an action was handled (and the app should exit).
    pub fn handle_cli_actions(&self) -> bool {
        let args: Vec<String> = std::env::args().collect();
        if let Some(action_idx) = args.iter().position(|a| a == "--rupa-action") {
            if let Some(action_name) = args.get(action_idx + 1) {
                let payload = if let Some(p_idx) = args.iter().position(|a| a == "--rupa-payload") {
                    args.get(p_idx + 1).cloned().unwrap_or_else(|| "{}".to_string())
                } else {
                    "{}".to_string()
                };

                let core = self.core.read().unwrap();
                if let Some(handler) = core.action_handlers.get(action_name) {
                    match handler.handle_json(&payload) {
                        Ok(_) => println!("✨ Action '{}' executed successfully.", action_name),
                        Err(e) => eprintln!("❌ Action '{}' failed: {}", action_name, e),
                    }
                } else {
                    eprintln!("❌ Error: No handler registered for action '{}'.", action_name);
                }
                return true;
            }
        }
        false
    }
}
