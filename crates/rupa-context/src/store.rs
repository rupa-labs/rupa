use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

/// A scoped registry for dependency injection that supports parent lookups.
#[derive(Clone, Default)]
pub struct Registry {
    parent: Option<Weak<Registry>>,
    data: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_parent(parent: Arc<Registry>) -> Self {
        Self {
            parent: Some(Arc::downgrade(&parent)),
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert<T: Send + Sync + 'static>(&self, value: T) {
        let mut map = self.data.write().unwrap();
        map.insert(TypeId::of::<T>(), Arc::new(value));
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        {
            let map = self.data.read().unwrap();
            if let Some(val) = map.get(&TypeId::of::<T>()) {
                return val.clone().downcast::<T>().ok();
            }
        }

        if let Some(ref weak_parent) = self.parent {
            if let Some(parent) = weak_parent.upgrade() {
                return parent.get::<T>();
            }
        }

        None
    }

    pub fn has<T: Send + Sync + 'static>(&self) -> bool {
        self.get::<T>().is_some()
    }
}

impl std::fmt::Debug for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("has_parent", &self.parent.is_some())
            .field("entries", &self.data.read().unwrap().len())
            .finish()
    }
}
