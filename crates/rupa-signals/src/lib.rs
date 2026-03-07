pub mod id;

use std::sync::{Arc, RwLock};
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};

static SIGNAL_ID: AtomicUsize = AtomicUsize::new(0);

/// A reactive state container.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Signal<T> {
    id: usize,
    value: Arc<RwLock<T>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: SIGNAL_ID.fetch_add(1, Ordering::SeqCst),
            value: Arc::new(RwLock::new(value)),
        }
    }

    pub fn get(&self) -> T {
        self.value.read().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        *self.value.write().unwrap() = value;
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let mut val = self.value.write().unwrap();
        f(&mut *val);
    }
}

pub struct Memo<T> {
    _id: usize,
    _value: Arc<RwLock<T>>,
}

pub struct Effect {
    _id: usize,
}

pub trait Readable<T> {
    fn get(&self) -> T;
}

impl<T: Clone> Readable<T> for Signal<T> {
    fn get(&self) -> T { self.get() }
}
