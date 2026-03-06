use std::sync::{Arc, RwLock};
use std::fmt::Debug;
use crate::platform::request_redraw;

pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    /// Callback to trigger when value changes. 
    /// This will be used to mark components as dirty.
    on_change: Arc<RwLock<Option<Box<dyn Fn() + Send + Sync>>>>,
}

impl<T: Clone + Debug + Send + Sync + 'static> Signal<T> {
    pub fn new(val: T) -> Self {
        Self { 
            value: Arc::new(RwLock::new(val)),
            on_change: Arc::new(RwLock::new(None)),
        }
    }

    /// Connects this signal to a specific update trigger (e.g., mark component dirty)
    pub fn connect(&self, f: impl Fn() + Send + Sync + 'static) {
        *self.on_change.write().unwrap() = Some(Box::new(f));
    }

    pub fn get(&self) -> T {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, val: T) {
        *self.value.write().unwrap() = val;
        self.trigger_change();
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut val = self.value.write().unwrap();
        f(&mut *val);
        drop(val); // Release lock before trigger
        self.trigger_change();
    }

    fn trigger_change(&self) {
        if let Some(ref f) = *self.on_change.read().unwrap() {
            (f)();
        }
        request_redraw();
    }
}

impl<T: Clone + Debug + Send + Sync + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { 
            value: self.value.clone(),
            on_change: self.on_change.clone(),
        }
    }
}

impl<T: Debug> Debug for Signal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal").field("value", &self.value).finish()
    }
}

pub struct Memo<T> {
    compute: Arc<dyn Fn() -> T + Send + Sync>,
}

impl<T: Clone + Debug + Send + Sync + 'static> Memo<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self { compute: Arc::new(f) }
    }

    pub fn get(&self) -> T {
        (self.compute)()
    }
}
