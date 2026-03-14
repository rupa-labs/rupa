//! # Rupa Signals 🧬
//!
//! Fine-grained reactivity engine for the Rupa Framework. 
//! Provides the reactive primitives: `Signal`, `Memo`, and `Effect`.

pub mod runtime;
pub mod signal;
pub mod memo;
pub mod effect;

pub use runtime::{batch, with_context, untrack, Subscriber};
pub use signal::Signal;
pub use memo::Memo;
pub use effect::Effect;

use serde::{Serialize, Deserialize};

/// Standard cursor icons supported across platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum CursorIcon {
    #[default] Default, Pointer, Text, Grab, Grabbing, NotAllowed, Wait, Crosshair,
}

/// A trait for types that can be read reactively.
pub trait Readable<T> {
    /// Gets the current value and establishes a reactive dependency.
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
    use std::sync::Arc;
    use std::sync::RwLock;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;

    #[test]
    fn test_signal_basic() {
        let s = Signal::new(10);
        assert_eq!(s.get(), 10);
        s.set(20);
        assert_eq!(s.get(), 20);
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
    fn test_batch_updates() {
        let s = Signal::new(0);
        let call_count = Arc::new(AtomicUsize::new(0));
        let _e = {
            let s = s.clone();
            let count = call_count.clone();
            Effect::new(move || {
                s.get();
                count.fetch_add(1, Ordering::SeqCst);
            })
        };

        assert_eq!(call_count.load(Ordering::SeqCst), 1);
        
        batch(|| {
            s.set(1);
            s.set(2);
            s.set(3);
        });

        assert_eq!(call_count.load(Ordering::SeqCst), 2);
    }

    #[test]
    fn test_cross_thread_reactivity() {
        let s = Signal::new(10);
        let out = Arc::new(RwLock::new(0));
        
        let _e = {
            let s = s.clone();
            let out = out.clone();
            Effect::new(move || {
                *out.write().unwrap() = s.get();
            })
        };

        let s_clone = s.clone();
        thread::spawn(move || {
            s_clone.set(50);
        }).join().unwrap();

        assert_eq!(*out.read().unwrap(), 50);
    }

    #[test]
    fn test_untrack_behavior() {
        let s = Signal::new(10);
        let count = Arc::new(AtomicUsize::new(0));
        
        let _e = {
            let s = s.clone();
            let count = count.clone();
            Effect::new(move || {
                untrack(|| s.get());
                count.fetch_add(1, Ordering::SeqCst);
            })
        };

        assert_eq!(count.load(Ordering::SeqCst), 1);
        s.set(20);
        assert_eq!(count.load(Ordering::SeqCst), 1); // Should not re-run
    }
}
