pub mod id;

use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize, Deserialize};

static SIGNAL_ID: AtomicUsize = AtomicUsize::new(0);

/// A reactive state container.
#[derive(Clone, Serialize, Deserialize)]
pub struct Signal<T> {
    id: usize,
    #[serde(skip)]
    value: Arc<RwLock<Option<T>>>,
}

impl<T: Clone> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            id: SIGNAL_ID.fetch_add(1, Ordering::SeqCst),
            value: Arc::new(RwLock::new(Some(value))),
        }
    }

    pub fn get(&self) -> T {
        self.value.read().unwrap().as_ref().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        *self.value.write().unwrap() = Some(value);
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        let mut guard = self.value.write().unwrap();
        if let Some(ref mut val) = *guard {
            f(val);
        }
    }
}

impl<T> Default for Signal<T> {
    fn default() -> Self {
        Self {
            id: SIGNAL_ID.fetch_add(1, Ordering::SeqCst),
            value: Arc::new(RwLock::new(None)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorIcon {
    #[default]
    Default,
    Pointer,
    Text,
    Grab,
    Grabbing,
    NotAllowed,
    Wait,
    Crosshair,
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
