//! # Rupa Full-Stack 🥞
//!
//! The Unified Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) that bind `rupa-server` and 
//! `rupa-web` together, enabling seamless Server-Side Rendering (SSR) 
//! with client-side hydration.

pub use rupa_core as core;
pub use rupa_ui as ui;
pub use rupa_server as server;
pub use rupa_web as web;
pub use rupa_router as router;
pub use rupa_auth as auth;
pub use rupa_signals as signals;
pub use rupa_store as data;
pub use rupa_net as net;
pub use rupa_i18n as i18n;

/// The standard prelude for building Rupa Full-Stack applications.
pub mod prelude {
    pub use rupa_core::{Component, Signal, Memo, Effect, Readable};
    pub use rupa_ui::elements::*;
    pub use rupa_server::HtmlRenderer;
    pub use rupa_router::{Router, Route, use_route};
    pub use rupa_auth::{User, Session};
    pub use rupa_store::{Store, PersistentSignal};
}
