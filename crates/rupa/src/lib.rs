//! # Rupa Framework 🎨
//!
//! The Universal Facade for the Rupa Framework. 
//! This crate provides a unified, ergonomic entry point that bundles 
//! all **Atoms** (Tier 1), **Composites** (Tier 2), and **Showrooms** (Tier 3) 
//! into a single dependency for rapid artisan development.

// --- Tier 1: Atoms (The Materials & Tools) ---
pub use rupa_signals as signals;
pub use rupa_vnode as vnode;
pub use rupa_store as store;
pub use rupa_net as net;
pub use rupa_auth as auth;
pub use rupa_i18n as i18n;
pub use rupa_motion as motion;
pub use rupa_assets as assets;

// --- Tier 2: Composites (The Master's Craft) ---
pub use rupa_core as core;
pub use rupa_ui as ui;
pub use rupa_engine as engine;
#[cfg(feature = "ssr")]
pub use rupa_server as server_core;
#[cfg(feature = "web")]
pub use rupa_web as web_core;
#[cfg(feature = "mobile")]
pub use rupa_mobile as mobile_core;
pub use rupa_macros as macros;


pub mod prelude;

// Global exports for convenience
pub use rupa_engine::App;
pub use rupa_ui::body::Body;
pub use rupa_signals::{Signal, Memo, Effect, Readable};
pub use rupa_vnode::{Style, Color, Scale, Unit};
pub use rupa_ui::style::{Theme, Variant, ColorMode};

/// Extension trait to provide ergonomic methods to the [App] orchestrator.
pub trait AppExt {
    /// Sets the root content of the application. 
    /// Automatically wraps the component in a [Body] for aesthetic management.
    fn root(self, component: impl rupa_core::component::Component + 'static) -> Self;
}

impl AppExt for App {
    fn root(self, component: impl rupa_core::component::Component + 'static) -> Self {
        let body = Body::new(component);
        self.root(std::sync::Arc::new(body))
    }
}
