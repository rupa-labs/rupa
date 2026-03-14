use std::sync::{Arc, RwLock};
use std::sync::atomic::Ordering;
use crate::runtime::{Subscriber, RUNTIME};
use crate::signal::NEXT_ID;

pub struct Effect {
    pub _id: usize,
    pub _inner: Arc<dyn Subscriber>,
}

pub struct EffectInner<F> {
    pub _id: usize,
    pub func: RwLock<F>,
}

impl<F: Fn() + Send + Sync + 'static> EffectInner<F> {
    pub fn execute(self: &Arc<Self>) {
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
