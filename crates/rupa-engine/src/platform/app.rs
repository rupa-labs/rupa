use std::sync::Arc;
use rupa_core::Component;
use crate::platform::context::{PlatformCore, SharedPlatformCore};

#[derive(Clone)]
pub struct AppMetadata {
    pub title: String,
    pub version: String,
    pub author: String,
}

pub struct App {
    pub core: SharedPlatformCore,
}

impl App {
    pub fn new(title: impl Into<String>) -> Self {
        let metadata = AppMetadata {
            title: title.into(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            author: "Rupa Artisan".to_string(),
        };

        Self {
            core: Arc::new(std::sync::RwLock::new(PlatformCore::new(metadata, None))),
        }
    }

    pub fn root(self, component: impl Component + 'static) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.root = Some(Box::new(component));
        }
        self
    }

    pub fn debug(self, enabled: bool) -> Self {
        {
            let mut core = self.core.write().unwrap();
            core.debug = enabled;
        }
        self
    }
}
