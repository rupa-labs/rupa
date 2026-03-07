pub mod infra;

use rupa_core::events::InputEvent;
use rupa_engine::App;

/// Standardized mobile lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileLifecycle {
    /// App is visible and in the foreground.
    Active,
    /// App is partially obscured or moving to background.
    Inactive,
    /// App is in the background, GPU resources should be released.
    Suspended,
}

/// The core trait for mobile-specific application runners.
pub trait MobileRunner {
    /// Initializes the native mobile shell.
    fn init(&mut self);
    
    /// Handles native lifecycle transitions.
    fn on_lifecycle_change(&mut self, state: MobileLifecycle);
    
    /// Dispatches touch events to the Rupa core.
    fn handle_input(&mut self, event: InputEvent);
}

/// Entry point for bootstrapping a Rupa application on mobile.
pub fn bootstrap_mobile(app: App) {
    log::info!("Rupa Mobile: Bootstrapping application...");
    // Initialization logic for Winit event loop on Android/iOS
}
