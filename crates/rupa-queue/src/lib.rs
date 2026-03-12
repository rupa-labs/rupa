pub mod task;
pub mod queue;

use std::sync::Arc;
pub use task::{Task, Status, Handle};
pub use queue::Queue;

pub type Port = Arc<dyn Task>;
