use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::fmt::Debug;
use crate::app::request_redraw;

pub struct Signal<T> { value: Arc<RwLock<T>>, version: Arc<AtomicU64> }
impl<T: Clone + Debug> Signal<T> {
    pub fn new(initial: T) -> Self { Self { value: Arc::new(RwLock::new(initial)), version: Arc::new(AtomicU64::new(0)) } }
    pub fn get(&self) -> T { self.value.read().unwrap().clone() }
    pub fn version(&self) -> u64 { self.version.load(Ordering::SeqCst) }
    pub fn set(&self, new_value: T) { let mut val = self.value.write().unwrap(); *val = new_value; self.version.fetch_add(1, Ordering::SeqCst); request_redraw(); }
    pub fn update<F>(&self, f: F) where F: FnOnce(&mut T) { let mut val = self.value.write().unwrap(); f(&mut val); self.version.fetch_add(1, Ordering::SeqCst); request_redraw(); }
}
impl<T: Clone> Clone for Signal<T> { fn clone(&self) -> Self { Self { value: Arc::clone(&self.value), version: Arc::clone(&self.version) } } }

pub struct Memo<T, D> { source: Signal<T>, derivation: Box<dyn Fn(&T) -> D>, cached_value: Arc<RwLock<D>>, last_version: AtomicU64 }
impl<T: Clone + Debug, D: Clone + Debug + PartialEq> Memo<T, D> {
    pub fn new(source: Signal<T>, derivation: impl Fn(&T) -> D + 'static) -> Self {
        let initial = derivation(&source.get());
        Self { source, derivation: Box::new(derivation), cached_value: Arc::new(RwLock::new(initial)), last_version: AtomicU64::new(0) }
    }
    pub fn get(&self) -> D {
        let current_ver = self.source.version();
        if current_ver != self.last_version.load(Ordering::SeqCst) {
            let mut cache = self.cached_value.write().unwrap();
            *cache = (self.derivation)(&self.source.get());
            self.last_version.store(current_ver, Ordering::SeqCst);
            cache.clone()
        } else { self.cached_value.read().unwrap().clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_signal_update() { let s = Signal::new(10); s.set(20); assert_eq!(s.get(), 20); }
    #[test]
    fn test_memo_derivation() { let s = Signal::new(10); let m = Memo::new(s.clone(), |v| v * 2); assert_eq!(m.get(), 20); s.set(20); assert_eq!(m.get(), 40); }
}
