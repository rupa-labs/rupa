use std::sync::{Arc, RwLock, Weak};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::RefCell;
use serde::{Serialize, Deserialize};

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default());
}

/// A subscriber that can be notified when a dependency changes.
pub trait Subscriber: Send + Sync {
    fn notify(&self);
}

#[derive(Default)]
struct Runtime {
    stack: Vec<Weak<dyn Subscriber>>,
}

impl Runtime {
    fn push_context(&mut self, sub: Weak<dyn Subscriber>) {
        self.stack.push(sub);
    }

    fn pop_context(&mut self) {
        self.stack.pop();
    }

    fn current_context(&self) -> Option<Arc<dyn Subscriber>> {
        self.stack.last().and_then(|w| w.upgrade())
    }
}

/// Run a closure in the context of a subscriber.
pub fn with_context<F, R>(sub: Arc<dyn Subscriber>, f: F) -> R
where F: FnOnce() -> R {
    RUNTIME.with(|rt| {
        rt.borrow_mut().push_context(Arc::downgrade(&sub));
        let res = f();
        rt.borrow_mut().pop_context();
        res
    })
}

/// Run a closure without any reactive tracking.
pub fn untrack<F, R>(f: F) -> R
where F: FnOnce() -> R {
    RUNTIME.with(|rt| {
        let prev = rt.borrow_mut().stack.split_off(0);
        let res = f();
        rt.borrow_mut().stack = prev;
        res
    })
}

// --- SIGNAL ---

pub struct SignalInner<T> {
    value: RwLock<T>,
    subscribers: RwLock<Vec<Weak<dyn Subscriber>>>,
}

/// A reactive state container.
pub struct Signal<T> {
    id: usize,
    inner: Arc<SignalInner<T>>,
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
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
            id: NEXT_ID.fetch_add(1, Ordering::SeqCst),
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

        for sub in subs {
            sub.notify();
        }
    }
}

// --- MEMO ---

struct MemoInner<T, F> {
    _id: usize,
    value: RwLock<Option<T>>,
    func: F,
    is_dirty: RwLock<bool>,
    subscribers: RwLock<Vec<Weak<dyn Subscriber>>>,
}

pub struct Memo<T> {
    inner: Arc<dyn MemoTrait<T>>,
}

impl<T> Clone for Memo<T> {
    fn clone(&self) -> Self {
        Self { inner: self.inner.clone() }
    }
}

trait MemoTrait<T>: Subscriber {
    fn get(&self) -> T;
}

impl<T: Clone + Send + Sync + 'static, F: Fn() -> T + Send + Sync + 'static> MemoTrait<T> for MemoInner<T, F> {
    fn get(&self) -> T {
        RUNTIME.with(|rt| {
            if let Some(sub) = rt.borrow().current_context() {
                let mut subs = self.subscribers.write().unwrap();
                subs.push(Arc::downgrade(&sub));
            }
        });

        let mut is_dirty = self.is_dirty.write().unwrap();
        if *is_dirty || self.value.read().unwrap().is_none() {
            let result = (self.func)();
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
        *self.is_dirty.write().unwrap() = true;
        let subs = self.subscribers.read().unwrap();
        for sub in subs.iter().filter_map(|w| w.upgrade()) {
            sub.notify();
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
        self.inner.get()
    }
}

// --- EFFECT ---

pub struct Effect {
    _id: usize,
    _inner: Arc<dyn Subscriber>,
}

struct EffectInner<F> {
    id: usize,
    func: F,
}

impl<F: Fn() + Send + Sync + 'static> Subscriber for EffectInner<F> {
    fn notify(&self) {
        // Safe because notify doesn't take &Arc<Self>, but we need to run it in context.
        // For simplicity in this artisan impl, we'll re-run via a helper.
    }
}

impl Effect {
    pub fn new<F>(func: F) -> Self 
    where F: Fn() + Send + Sync + 'static {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let inner = Arc::new(EffectInner { id, func });
        
        // Initial run
        RUNTIME.with(|rt| {
            let sub: Arc<dyn Subscriber> = inner.clone();
            rt.borrow_mut().push_context(Arc::downgrade(&sub));
            (inner.func)();
            rt.borrow_mut().pop_context();
        });

        Effect { 
            _id: id,
            _inner: inner,
        }
    }
}

// --- UTILS ---

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorIcon {
    #[default] Default, Pointer, Text, Grab, Grabbing, NotAllowed, Wait, Crosshair,
}

pub trait Readable<T> {
    fn get(&self) -> T;
}

impl<T: Clone + Send + Sync> Readable<T> for Signal<T> {
    fn get(&self) -> T { self.get() }
}

impl<T: Clone + Send + Sync + 'static> Readable<T> for Memo<T> {
    fn get(&self) -> T { self.get() }
}
