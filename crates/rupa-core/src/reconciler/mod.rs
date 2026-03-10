pub mod patch;
pub mod diff;

pub use patch::{Patch, UpdateType};
pub use diff::reconcile;

/// The result of a reconciliation process.
pub type PatchSet = Vec<Patch>;
