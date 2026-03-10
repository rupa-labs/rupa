use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A registry that holds type-erased reactive signals for a specific tree scope.
#[derive(Default, Clone)]
pub struct ContextRegistry {
    data: Arc<RwLock<HashMap<TypeId, Arc<dyn Any + Send + Sync>>>>,
}

impl ContextRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: Send + Sync + 'static>(&self, value: T) {
        let mut map = self.data.write().unwrap();
        map.insert(TypeId::of::<T>(), Arc::new(value));
    }

    pub fn get<T: Send + Sync + 'static>(&self) -> Option<Arc<T>> {
        let map = self.data.read().unwrap();
        map.get(&TypeId::of::<T>()).and_then(|v| v.clone().downcast::<T>().ok())
    }
}
