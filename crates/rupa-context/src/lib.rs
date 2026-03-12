pub mod provider;
pub mod consumer;
pub mod store;

use std::sync::Arc;
use std::cell::RefCell;
pub use store::Registry;

thread_local! {
    static CURRENT_REGISTRY: RefCell<Arc<Registry>> = RefCell::new(Arc::new(Registry::new()));
}

/// Provides a value to the current context scope.
pub fn provide_context<T: Send + Sync + 'static>(value: T) {
    CURRENT_REGISTRY.with(|reg| {
        reg.borrow().insert(value);
    });
}

/// Retrieves a value from the current context scope or any parent scope.
pub fn use_context<T: Send + Sync + 'static>() -> Option<Arc<T>> {
    CURRENT_REGISTRY.with(|reg| {
        reg.borrow().get::<T>()
    })
}

/// Executes a closure within a specific context scope.
pub fn with_registry<F, R>(registry: Arc<Registry>, f: F) -> R
where F: FnOnce() -> R {
    let prev = CURRENT_REGISTRY.with(|reg| {
        let mut b = reg.borrow_mut();
        let prev = b.clone();
        *b = registry;
        prev
    });
    
    let res = f();
    
    CURRENT_REGISTRY.with(|reg| {
        *reg.borrow_mut() = prev;
    });
    
    res
}

/// Creates a new scoped registry based on the current context.
pub fn create_scoped_registry() -> Arc<Registry> {
    CURRENT_REGISTRY.with(|reg| {
        Arc::new(Registry::with_parent(reg.borrow().clone()))
    })
}
