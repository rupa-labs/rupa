use std::sync::{Arc, RwLock, Weak};
use std::sync::atomic::{AtomicUsize, Ordering};
use serde::{Serialize, Deserialize};
use crate::runtime::{Subscriber, RUNTIME};

pub static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

pub struct SignalInner<T> {
    pub value: RwLock<T>,
    pub subscribers: RwLock<Vec<Weak<dyn Subscriber>>>,
}

pub struct Signal<T> {
    pub _id: usize,
    pub inner: Arc<SignalInner<T>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            _id: self._id,
            inner: self.inner.clone(),
        }
    }
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + Send + Sync> Serialize for Signal<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        self.get().serialize(serializer)
    }
}

impl<'de, T: Serialize + Deserialize<'de> + Clone + Send + Sync> Deserialize<'de> for Signal<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let value = T::deserialize(deserializer)?;
        Ok(Signal::new(value))
    }
}

impl<T: Clone + Send + Sync> Signal<T> {
    pub fn new(value: T) -> Self {
        Self {
            _id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
            inner: Arc::new(SignalInner {
                value: RwLock::new(value),
                subscribers: RwLock::new(Vec::new()),
            }),
        }
    }

    pub fn get(&self) -> T {
        RUNTIME.with(|rt| {
            if let Some(sub) = rt.borrow().current_context() {
                let mut subs = self.inner.subscribers.write().unwrap();
                subs.push(Arc::downgrade(&sub));
            }
        });
        self.inner.value.read().unwrap().clone()
    }

    pub fn set(&self, value: T) {
        {
            let mut val = self.inner.value.write().unwrap();
            *val = value;
        }
        self.notify_subscribers();
    }

    pub fn update(&self, f: impl FnOnce(&mut T)) {
        {
            let mut val = self.inner.value.write().unwrap();
            f(&mut *val);
        }
        self.notify_subscribers();
    }

    fn notify_subscribers(&self) {
        let subs = {
            let mut subs_guard = self.inner.subscribers.write().unwrap();
            let alive: Vec<Arc<dyn Subscriber>> = subs_guard
                .iter()
                .filter_map(|w| w.upgrade())
                .collect();
            *subs_guard = alive.iter().map(|a| Arc::downgrade(a)).collect();
            alive
        };

        RUNTIME.with(|rt| {
            let mut runtime = rt.borrow_mut();
            if runtime.batch_depth > 0 {
                for sub in subs {
                    if !runtime.pending_notifications.iter().any(|p| Arc::ptr_eq(p, &sub)) {
                        runtime.pending_notifications.push(sub);
                    }
                }
            } else {
                for sub in subs {
                    sub.notify();
                }
            }
        });
    }
}
