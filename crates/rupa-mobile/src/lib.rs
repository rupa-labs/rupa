//! # Rupa Mobile 📱
//!
//! The Mobile Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) to run Rupa applications 
//! natively on iOS and Android using Winit and specific NDK bindings.

pub mod infra;

pub use rupa_core::events::InputEvent;
pub use rupa_engine::App;

/// Standardized mobile lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileLifecycle {
    /// App is visible and in the foreground.
    Active,
    /// App is in the foreground but not receiving events.
    Inactive,
    /// App is in the background.
    Background,
}

/// Unified access to mobile-specific hardware features.
pub struct MobileHardware;

impl MobileHardware {
    /// Triggers the device haptic motor.
    pub fn vibrate(duration_ms: u32) {
        log::info!("Rupa Mobile: Vibrate for {}ms", duration_ms);
    }
}

/// Entry point for bootstrapping a Rupa application on mobile.
pub fn bootstrap_mobile(__app: App) {
    log::info!("Rupa Mobile: Bootstrapping application...");
    // Initialization logic for Winit event loop on Android/iOS
}
