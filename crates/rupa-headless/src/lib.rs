//! # Rupa Headless 👻
//!
//! The Headless Showroom for the Rupa Framework. This crate provides the 
//! **Adapters & Infrastructure** (Tier 3) to run Rupa applications 
//! without a physical display. It is optimized for background services, 
//! automation scripts, and high-performance server logic.

pub use rupa_core as core;
pub use rupa_signals as signals;
pub use rupa_vnode as vnode;
pub use rupa_store as data;
pub use rupa_net as net;

/// The standard prelude for building Rupa Headless applications.
pub mod prelude {
    pub use rupa_core::{Signal, Memo, Effect, Readable};
    pub use rupa_vnode::VNode;
    pub use rupa_store::{Store, PersistentSignal};
    pub use rupa_net::{Client, Resource};
}
