use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::fmt::Debug;

/// A reactive state container that notifies listeners on change.
pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    version: Arc<AtomicU64>,
}

impl<T: Clone + Debug> Signal<T> {
    pub fn new(initial: T) -> Self {
        Self {
            value: Arc::new(RwLock::new(initial)),
            version: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn get(&self) -> T {
        self.value.read().unwrap().clone()
    }

    pub fn version(&self) -> u64 {
        self.version.load(Ordering::SeqCst)
    }

    pub fn set(&self, new_value: T) {
        let mut val = self.value.write().unwrap();
        *val = new_value;
        self.version.fetch_add(1, Ordering::SeqCst);
        log::trace!("Signal updated (v{}): {:?}", self.version(), *val);
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut val = self.value.write().unwrap();
        f(&mut val);
        self.version.fetch_add(1, Ordering::SeqCst);
        log::trace!("Signal mutated (v{}): {:?}", self.version(), *val);
    }
}

impl<T: Clone> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            version: Arc::clone(&self.version),
        }
    }
}

/// A derived state that caches its value until dependencies change.
/// This prevents unnecessary recalculations and memory pressure (Memoization).
pub struct Memo<T, D> {
    source: Signal<D>,
    compute: Box<dyn Fn(D) -> T>,
    cached_value: Arc<RwLock<T>>,
    last_version: Arc<AtomicU64>,
}

impl<T: Clone + Debug, D: Clone + Debug> Memo<T, D> {
    /// Creates a new derived state (Memo).
    pub fn new(source: Signal<D>, compute: impl Fn(D) -> T + 'static) -> Self {
        let initial_val = compute(source.get());
        let initial_ver = source.version();
        
        Self {
            source,
            compute: Box::new(compute),
            cached_value: Arc::new(RwLock::new(initial_val)),
            last_version: Arc::new(AtomicU64::new(initial_ver)),
        }
    }

    /// Returns the cached value, or recalculates it if the source has changed.
    pub fn get(&self) -> T {
        let current_ver = self.source.version();
        let last_ver = self.last_version.load(Ordering::SeqCst);

        if current_ver > last_ver {
            // Cache is dirty, recalculate
            let mut cache = self.cached_value.write().unwrap();
            *cache = (self.compute)(self.source.get());
            self.last_version.store(current_ver, Ordering::SeqCst);
            log::trace!("Memo recalculated (v{}): {:?}", current_ver, *cache);
            cache.clone()
        } else {
            // Cache is fresh, return immediately
            self.cached_value.read().unwrap().clone()
        }
    }
}
