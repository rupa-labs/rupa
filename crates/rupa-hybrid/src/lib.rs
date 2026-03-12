//! # Rupa Hybrid 🌉
//!
//! The Hybrid Interoperability Showroom for the Rupa Framework. 
//! This crate provides the **Adapters & Infrastructure** (Tier 3) required 
//! to embed Rupa Web applications inside native wrappers (like Tauri, Tauri-mobile, 
//! or Electron) while maintaining seamless communication with the native host.

pub use rupa_core as core;
pub use rupa_vnode as vnode;
pub use rupa_web as client;

/// The standard prelude for building Rupa Hybrid applications.
pub mod prelude {
    pub use rupa_core::{Component, Signal, Readable};
    pub use rupa_vnode::VNode;
    pub use rupa_web::RupaApp;
}
