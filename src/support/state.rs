use std::sync::{Arc, RwLock, atomic::{AtomicU64, AtomicU32, Ordering}};
use std::fmt::Debug;
use std::cell::Cell;
use crate::platform::request_redraw;

thread_local! {
    /// Tracks if we are currently in a batch update to suppress multiple redraws.
    static BATCH_COUNT: Cell<u32> = Cell::new(0);
}

/// A trait for any reactive value that can be read.
pub trait Readable<T> {
    fn get(&self) -> T;
    fn version(&self) -> u64;
}

/// Executes a closure and ensures only a single redraw request is sent 
/// regardless of how many signals were mutated inside.
pub fn batch<F, R>(f: F) -> R 
where F: FnOnce() -> R {
    BATCH_COUNT.with(|count| count.set(count.get() + 1));
    let result = f();
    BATCH_COUNT.with(|count| {
        let new_count = count.get() - 1;
        count.set(new_count);
        if new_count == 0 {
            request_redraw();
        }
    });
    result
}

fn notify_change() {
    let in_batch = BATCH_COUNT.with(|count| count.get() > 0);
    if !in_batch {
        request_redraw();
    }
}

// --- SIGNAL ---

pub struct Signal<T> {
    value: Arc<RwLock<T>>,
    version: Arc<AtomicU64>,
    on_change: Arc<RwLock<Option<Box<dyn Fn() + Send + Sync>>>>,
}

impl<T: Clone + Debug + Send + Sync + 'static> Signal<T> {
    pub fn new(val: T) -> Self {
        Self { 
            value: Arc::new(RwLock::new(val)),
            version: Arc::new(AtomicU64::new(1)),
            on_change: Arc::new(RwLock::new(None)),
        }
    }

    pub fn connect(&self, f: impl Fn() + Send + Sync + 'static) {
        *self.on_change.write().unwrap() = Some(Box::new(f));
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
        drop(val);
        self.trigger_change();
    }

    fn trigger_change(&self) {
        self.version.fetch_add(1, Ordering::SeqCst);
        if let Some(ref f) = *self.on_change.read().unwrap() {
            (f)();
        }
        notify_change();
    }
}

impl<T: Clone + Debug + Send + Sync + 'static> Readable<T> for Signal<T> {
    fn get(&self) -> T {
        self.value.read().unwrap().clone()
    }
    fn version(&self) -> u64 {
        self.version.load(Ordering::SeqCst)
    }
}

impl<T: Clone + Debug + Send + Sync + 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self { 
            value: self.value.clone(),
            version: self.version.clone(),
            on_change: self.on_change.clone(),
        }
    }
}

// --- MEMO ---

pub struct Memo<T> {
    compute: Arc<dyn Fn() -> T + Send + Sync>,
    cache: Arc<RwLock<Option<(T, u64)>>>,
}

impl<T: Clone + Debug + Send + Sync + 'static> Memo<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn() -> T + Send + Sync + 'static,
    {
        Self { 
            compute: Arc::new(f),
            cache: Arc::new(RwLock::new(None)),
        }
    }
}

impl<T: Clone + Debug + Send + Sync + 'static> Readable<T> for Memo<T> {
    fn get(&self) -> T {
        // Memos in v0 are computed on demand. 
        // Real version-based caching will be implemented in the next step.
        (self.compute)()
    }
    fn version(&self) -> u64 { 0 }
}

impl<T: Clone + Debug + Send + Sync + 'static> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self {
            compute: self.compute.clone(),
            cache: self.cache.clone(),
        }
    }
}
