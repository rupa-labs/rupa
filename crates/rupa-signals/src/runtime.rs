use std::sync::{Arc, Weak};
use std::cell::RefCell;

pub trait Subscriber: Send + Sync {
    fn notify(&self);
    fn run(&self) {}
}

#[derive(Default)]
pub struct Runtime {
    pub stack: Vec<Weak<dyn Subscriber>>,
    pub batch_depth: usize,
    pub pending_notifications: Vec<Arc<dyn Subscriber>>,
}

thread_local! {
    pub static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::default());
}

impl Runtime {
    pub fn push_context(&mut self, sub: Weak<dyn Subscriber>) {
        self.stack.push(sub);
    }

    pub fn pop_context(&mut self) {
        self.stack.pop();
    }

    pub fn current_context(&self) -> Option<Arc<dyn Subscriber>> {
        self.stack.last().and_then(|w| w.upgrade())
    }

    pub fn start_batch(&mut self) {
        self.batch_depth += 1;
    }

    pub fn end_batch(&mut self) {
        self.batch_depth -= 1;
        if self.batch_depth == 0 {
            let pending = std::mem::take(&mut self.pending_notifications);
            for sub in pending {
                sub.notify();
            }
        }
    }
}

pub fn batch<F, R>(f: F) -> R
where F: FnOnce() -> R {
    RUNTIME.with(|rt| {
        rt.borrow_mut().start_batch();
        let res = f();
        rt.borrow_mut().end_batch();
        res
    })
}

pub fn with_context<F, R>(sub: Arc<dyn Subscriber>, f: F) -> R
where F: FnOnce() -> R {
    RUNTIME.with(|rt| {
        rt.borrow_mut().push_context(Arc::downgrade(&sub));
        let res = f();
        rt.borrow_mut().pop_context();
        res
    })
}

pub fn untrack<F, R>(f: F) -> R
where F: FnOnce() -> R {
    RUNTIME.with(|rt| {
        let prev = rt.borrow_mut().stack.split_off(0);
        let res = f();
        rt.borrow_mut().stack = prev;
        res
    })
}
