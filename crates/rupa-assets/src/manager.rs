use std::sync::Arc;
use tokio::sync::RwLock;
use rupa_base::Error;
use rupa_signals::Signal;
use crate::loader::Loader;
use crate::cache::Cache;
use rupa_queue::{Task, Queue};
use async_trait::async_trait;

/// The central orchestrator for all assets.
pub struct Manager {
    loaders: Arc<RwLock<Vec<Box<dyn Loader>>>>,
    cache: Arc<Cache>,
    pub stats: Signal<Stats>,
    pub queue: Option<Arc<Queue>>,
}

/// A background task for loading an asset.
pub struct LoadTask {
    pub path: String,
    pub manager: Arc<Manager>,
}

#[async_trait]
impl Task for LoadTask {
    fn name(&self) -> &str { &self.path }
    async fn run(&self) -> Result<(), Error> {
        let _ = self.manager.load_raw(&self.path).await?;
        Ok(())
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Stats {
    pub loaded_count: usize,
    pub cache_hits: usize,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            loaders: Arc::new(RwLock::new(Vec::new())),
            cache: Arc::new(Cache::new()),
            stats: Signal::new(Stats::default()),
            queue: None,
        }
    }

    pub fn with_queue(mut self, queue: Arc<Queue>) -> Self {
        self.queue = Some(queue);
        self
    }

    pub async fn register_loader(&self, loader: Box<dyn Loader>) {
        let mut loaders: tokio::sync::RwLockWriteGuard<'_, Vec<Box<dyn Loader>>> = self.loaders.write().await;
        loaders.push(loader);
    }

    pub async fn load_raw(&self, path: &str) -> Result<Vec<u8>, Error> {
        if let Some(data) = self.cache.get(path) {
            self.stats.update(|s| s.cache_hits += 1);
            return Ok(data);
        }

        let loaders: tokio::sync::RwLockReadGuard<'_, Vec<Box<dyn Loader>>> = self.loaders.read().await;
        for loader in loaders.iter() {
            if loader.can_handle(path) {
                match loader.load(path).await {
                    Ok(data) => {
                        self.cache.insert(path, data.clone());
                        self.stats.update(|s| s.loaded_count += 1);
                        return Ok(data);
                    }
                    Err(e) => return Err(e),
                }
            }
        }

        Err(Error::Resource(format!("No loader found for path: {}", path)))
    }

    pub fn load_queued(self: Arc<Self>, path: impl Into<String>) -> Result<(), Error> {
        if let Some(ref queue) = self.queue {
            let task = LoadTask {
                path: path.into(),
                manager: self.clone(),
            };
            queue.push(task);
            Ok(())
        } else {
            Err(Error::Resource("No queue configured for Manager".to_string()))
        }
    }
}
