// --- Atomic Materials (Tier 1) ---
pub use rupa_signals as signals;
pub use rupa_vnode as vnode;
pub use rupa_store as store;
pub use rupa_net as net;
pub use rupa_auth as auth;
pub use rupa_i18n as i18n;
pub use rupa_motion as motion;
pub use rupa_assets as assets;

// --- Composite Assemblies (Tier 2) ---
pub use rupa_core as core;
pub use rupa_ui as ui;
pub use rupa_engine as engine;
#[cfg(feature = "ssr")]
pub use rupa_server_core as server_core;
#[cfg(feature = "web")]
pub use rupa_web_core as web_core;
#[cfg(feature = "mobile")]
pub use rupa_mobile_core as mobile_core;
pub use rupa_macros as macros;


pub mod prelude;

// Global exports for convenience
pub use rupa_engine::App;
pub use rupa_ui::Body;
pub use rupa_signals::{Signal, Memo, Effect, Readable};
pub use rupa_vnode::{Style, Color};
pub use rupa_ui::style::{Theme, Variant, ColorMode};
