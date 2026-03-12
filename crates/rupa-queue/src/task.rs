use rupa_signals::Signal;
use rupa_base::Error;
use std::time::Instant;
use async_trait::async_trait;

/// The primary Port for defining background operations.
#[async_trait]
pub trait Task: Send + Sync + 'static {
    fn name(&self) -> &str;
    async fn run(&self) -> Result<(), Error>;
}

/// The current lifecycle state of a task.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Status {
    #[default]
    Pending,
    Running,
    Completed,
    Failed(Error),
}

/// A reactive handle for monitoring a specific task.
#[derive(Clone)]
pub struct Handle {
    pub name: String,
    pub status: Signal<Status>,
    pub progress: Signal<f32>,
    pub created_at: Instant,
}

impl Handle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: Signal::new(Status::Pending),
            progress: Signal::new(0.0),
            created_at: Instant::now(),
        }
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status.get(), Status::Running)
    }

    pub fn is_finished(&self) -> bool {
        matches!(self.status.get(), Status::Completed | Status::Failed(_))
    }
}
