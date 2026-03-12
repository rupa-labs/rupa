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
    /// Allows the subscriber to re-run itself if it holds logic (like an Effect).
    fn run(&self) {}
}

#[derive(Default)]
struct Runtime {
    stack: Vec<Weak<dyn Subscriber>>,
    batch_depth: usize,
    pending_notifications: Vec<Arc<dyn Subscriber>>,
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

    fn start_batch(&mut self) {
        self.batch_depth += 1;
    }

    fn end_batch(&mut self) {
        self.batch_depth -= 1;
        if self.batch_depth == 0 {
            let pending = std::mem::take(&mut self.pending_notifications);
            for sub in pending {
                sub.notify();
            }
        }
    }
}

/// Executes multiple state updates in a single batch, preventing redundant effects.
pub fn batch<F, R>(f: F) -> R
where F: FnOnce() -> R {
    RUNTIME.with(|rt| {
        rt.borrow_mut().start_batch();
        let res = f();
        rt.borrow_mut().end_batch();
        res
    })
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
    _id: usize,
    inner: Arc<SignalInner<T>>,
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
    fn get(self: Arc<Self>) -> T;
}

impl<T: Clone + Send + Sync + 'static, F: Fn() -> T + Send + Sync + 'static> MemoTrait<T> for MemoInner<T, F> {
    fn get(self: Arc<Self>) -> T {
        // Track the memo as a dependency for the current context
        RUNTIME.with(|rt| {
            if let Some(sub) = rt.borrow().current_context() {
                let mut subs = self.subscribers.write().unwrap();
                subs.push(Arc::downgrade(&sub));
            }
        });

        let mut is_dirty = self.is_dirty.write().unwrap();
        if *is_dirty || self.value.read().unwrap().is_none() {
            // Run the computation in the memo's own context to track its dependencies
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
            !was_dirty // Only notify if it wasn't already dirty
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

// --- EFFECT ---

pub struct Effect {
    _id: usize,
    _inner: Arc<dyn Subscriber>,
}

struct EffectInner<F> {
    _id: usize,
    func: RwLock<F>,
}

impl<F: Fn() + Send + Sync + 'static> EffectInner<F> {
    fn execute(self: &Arc<Self>) {
        RUNTIME.with(|rt| {
            let sub: Arc<dyn Subscriber> = self.clone();
            rt.borrow_mut().push_context(Arc::downgrade(&sub));
            (self.func.read().unwrap())();
            rt.borrow_mut().pop_context();
        });
    }
}

impl<F: Fn() + Send + Sync + 'static> Subscriber for EffectInner<F> {
    fn notify(&self) {
        // In this simplified implementation, we run directly.
        // A production implementation would use a global task queue.
        (self.func.read().unwrap())();
    }
}

impl Effect {
    pub fn new<F>(func: F) -> Self 
    where F: Fn() + Send + Sync + 'static {
        let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
        let inner = Arc::new(EffectInner { _id: id, func: RwLock::new(func) });
        
        inner.execute();

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn test_signal_basic() {
        let s = Signal::new(10);
        assert_eq!(s.get(), 10);
        s.set(20);
        assert_eq!(s.get(), 20);
    }

    #[test]
    fn test_signal_update() {
        let s = Signal::new(vec![1, 2]);
        s.update(|v| v.push(3));
        assert_eq!(s.get(), vec![1, 2, 3]);
    }

    #[test]
    fn test_memo_basic() {
        let s = Signal::new(10);
        let m = Memo::new({
            let s = s.clone();
            move || s.get() * 2
        });

        assert_eq!(m.get(), 20);
        s.set(15);
        assert_eq!(m.get(), 30);
    }

    #[test]
    fn test_memo_lazy() {
        let call_count = Arc::new(AtomicUsize::new(0));
        let s = Signal::new(10);
        let m = Memo::new({
            let s = s.clone();
            let count = call_count.clone();
            move || {
                count.fetch_add(1, Ordering::SeqCst);
                s.get() * 2
            }
        });

        // Should not have been called yet (lazy)
        assert_eq!(call_count.load(Ordering::SeqCst), 0);
        
        assert_eq!(m.get(), 20);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        // Accessing again without change shouldn't re-run
        assert_eq!(m.get(), 20);
        assert_eq!(call_count.load(Ordering::SeqCst), 1);

        s.set(20);
        // Dirty now, but still shouldn't run until 'get'
        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        assert_eq!(m.get(), 40);
        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_effect_basic() {
        let s = Signal::new(10);
        let out = Arc::new(RwLock::new(0));
        
        let _e = {
            let s = s.clone();
            let out = out.clone();
            Effect::new(move || {
                *out.write().unwrap() = s.get() + 1;
            })
        };

        assert_eq!(*out.read().unwrap(), 11);
        s.set(20);
        assert_eq!(*out.read().unwrap(), 21);
    }

    #[test]
    fn test_nested_reactivity() {
        let s = Signal::new(10);
        let m = Memo::new({
            let s = s.clone();
            move || s.get() + 5
        });
        let out = Arc::new(RwLock::new(0));

        let _e = {
            let m = m.clone();
            let out = out.clone();
            Effect::new(move || {
                *out.write().unwrap() = m.get() * 2;
            })
        };

        assert_eq!(*out.read().unwrap(), 30); // (10 + 5) * 2
        s.set(20);
        assert_eq!(*out.read().unwrap(), 50); // (20 + 5) * 2
    }

    #[test]
    fn test_untrack() {
        let s = Signal::new(10);
        let out = Arc::new(RwLock::new(0));

        let _e = {
            let s = s.clone();
            let out = out.clone();
            Effect::new(move || {
                let val = untrack(|| s.get());
                *out.write().unwrap() = val;
            })
        };

        assert_eq!(*out.read().unwrap(), 10);
        s.set(20);
        // Should NOT update because s.get() was untracked
        assert_eq!(*out.read().unwrap(), 10);
    }
}
