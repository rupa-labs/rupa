use rupa_signals::Signal;
use rupa_base::Error;
use std::future::Future;
use rupa_queue::{Task, Queue};
use std::sync::Arc;
use async_trait::async_trait;

/// A reactive state machine for managing asynchronous data operations.
#[derive(Clone, Debug, PartialEq, Default)]
pub enum Resource<T> {
    #[default]
    Uninitialized,
    Loading,
    Ready(T),
    Error(Error),
}

/// A background task for fetching data.
pub struct Fetch<T: Send + Sync + Clone + 'static> {
    pub name: String,
    pub signal: Signal<Resource<T>>,
    pub future: Arc<tokio::sync::Mutex<Option<Box<dyn Future<Output = Result<T, Error>> + Send + Unpin + 'static>>>>,
}

#[async_trait]
impl<T: Send + Sync + Clone + 'static> Task for Fetch<T> {
    fn name(&self) -> &str { &self.name }
    async fn run(&self) -> Result<(), Error> {
        let mut future_opt = self.future.lock().await;
        if let Some(future) = future_opt.take() {
            match future.await {
                Ok(data) => self.signal.set(Resource::Ready(data)),
                Err(err) => self.signal.set(Resource::Error(err)),
            }
        }
        Ok(())
    }
}

impl<T: Clone + Send + Sync + 'static> Resource<T> {
    pub fn new_signal() -> Signal<Self> {
        Signal::new(Resource::Uninitialized)
    }

    pub fn loading_signal() -> Signal<Self> {
        Signal::new(Resource::Loading)
    }

    pub fn fetch<F>(signal: Signal<Self>, future: F)
    where
        F: Future<Output = Result<T, Error>> + Send + 'static,
    {
        signal.set(Resource::Loading);
        
        tokio::spawn(async move {
            match future.await {
                Ok(data) => signal.set(Resource::Ready(data)),
                Err(err) => signal.set(Resource::Error(err)),
            }
        });
    }

    pub fn fetch_queued<F>(queue: &Queue, name: impl Into<String>, signal: Signal<Self>, future: F)
    where
        F: Future<Output = Result<T, Error>> + Send + Unpin + 'static,
    {
        signal.set(Resource::Loading);
        let task = Fetch {
            name: name.into(),
            signal,
            future: Arc::new(tokio::sync::Mutex::new(Some(Box::new(future)))),
        };
        queue.push(task);
    }

    pub fn is_loading(&self) -> bool { matches!(self, Resource::Loading) }
    pub fn is_ready(&self) -> bool { matches!(self, Resource::Ready(_)) }
    pub fn is_error(&self) -> bool { matches!(self, Resource::Error(_)) }

    pub fn data(&self) -> Option<&T> {
        match self {
            Resource::Ready(data) => Some(data),
            _ => None,
        }
    }

    pub fn error(&self) -> Option<&Error> {
        match self {
            Resource::Error(err) => Some(err),
            _ => None,
        }
    }
}
