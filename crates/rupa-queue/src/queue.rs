use std::sync::Arc;
use tokio::sync::mpsc;
use rupa_signals::Signal;
use crate::{Task, Handle, Status};

/// A reactive asynchronous task queue.
pub struct Queue {
    tasks: Signal<Vec<Handle>>,
    _sender: mpsc::UnboundedSender<Arc<dyn Task>>,
}

impl Queue {
    /// Creates a new asynchronous task queue.
    pub fn new(max_concurrency: usize) -> Arc<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Arc<dyn Task>>();
        let tasks = Signal::new(Vec::new());
        
        let queue = Arc::new(Self {
            tasks: tasks.clone(),
            _sender: tx,
        });

        // Spawn the queue worker
        let tasks_clone = tasks.clone();
        tokio::spawn(async move {
            let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrency));

            while let Some(task) = rx.recv().await {
                let permit = semaphore.clone().acquire_owned().await.unwrap();
                let handle = Handle::new(task.name());
                
                tasks_clone.update(|v| v.push(handle.clone()));
                
                tokio::spawn(async move {
                    handle.status.set(Status::Running);
                    
                    match task.run().await {
                        Ok(_) => handle.status.set(Status::Completed),
                        Err(e) => handle.status.set(Status::Failed(e)),
                    }
                    
                    handle.progress.set(1.0);
                    drop(permit);
                });
            }
        });

        queue
    }

    /// Pushes a new task into the queue.
    pub fn push(&self, task: impl Task) {
        let _ = self._sender.send(Arc::new(task));
    }

    /// Returns a reactive list of all tasks in the queue.
    pub fn tasks(&self) -> Signal<Vec<Handle>> {
        self.tasks.clone()
    }

    /// Clears all finished tasks from the queue.
    pub fn clear_finished(&self) {
        self.tasks.update(|v| {
            v.retain(|t| !t.is_finished());
        });
    }
}
