use std::sync::{Arc, RwLock, Weak};
use std::sync::atomic::Ordering;
use serde::{Serialize, Deserialize};
use crate::runtime::{Subscriber, RUNTIME, with_context};
use crate::signal::NEXT_ID;

pub struct MemoInner<T, F> {
    pub _id: usize,
    pub value: RwLock<Option<T>>,
    pub func: F,
    pub is_dirty: RwLock<bool>,
    pub subscribers: RwLock<Vec<Weak<dyn Subscriber>>>,
}

pub struct Memo<T> {
    pub inner: Arc<dyn MemoTrait<T>>,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync + 'static> Serialize for Memo<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.get().serialize(serializer)
    }
}

impl<'de, T: Serialize + Deserialize<'de> + Clone + Send + Sync + 'static> Deserialize<'de> for Memo<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value = T::deserialize(deserializer)?;
        Ok(Memo::new(move || value.clone()))
    }
}

pub trait MemoTrait<T>: Subscriber {
    fn get(self: Arc<Self>) -> T;
}

impl<T: Clone + Send + Sync + 'static, F: Fn() -> T + Send + Sync + 'static> MemoTrait<T> for MemoInner<T, F> {
    fn get(self: Arc<Self>) -> T {
        RUNTIME.with(|rt| {
            if let Some(sub) = rt.borrow().current_context() {
                let mut subs = self.subscribers.write().unwrap();
                subs.push(Arc::downgrade(&sub));
            }
        });

        let mut is_dirty = self.is_dirty.write().unwrap();
        if *is_dirty || self.value.read().unwrap().is_none() {
            let sub: Arc<dyn Subscriber> = self.clone();
            let result = with_context(sub, || (self.func)());
            
            *self.value.write().unwrap() = Some(result.clone());
            *is_dirty = false;
            result
        } else {
            self.value.read().unwrap().as_ref().unwrap().clone()
        }
    }
}

impl<T: Send + Sync, F: Send + Sync> Subscriber for MemoInner<T, F> {
    fn notify(&self) {
        let should_notify = {
            let mut is_dirty = self.is_dirty.write().unwrap();
            let was_dirty = *is_dirty;
            *is_dirty = true;
            !was_dirty
        };

        if should_notify {
            let subs = self.subscribers.read().unwrap();
            for sub in subs.iter().filter_map(|w| w.upgrade()) {
                sub.notify();
            }
        }
    }
}

impl<T: Clone + Send + Sync + 'static> Memo<T> {
    pub fn new<F>(func: F) -> Self 
    where F: Fn() -> T + Send + Sync + 'static {
        let inner = Arc::new(MemoInner {
            _id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            value: RwLock::new(None),
            func,
            is_dirty: RwLock::new(true),
            subscribers: RwLock::new(Vec::new()),
        });
        Self { inner }
    }

    pub fn get(&self) -> T {
        self.inner.clone().get()
    }
}
