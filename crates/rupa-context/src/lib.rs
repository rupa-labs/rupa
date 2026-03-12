//! # Rupa Context 💉
//!
//! Dependency Injection and Scoped Registry system for the Rupa Framework. 
//! Enables decoupled resource sharing across the component tree.

pub mod provider;
pub mod consumer;
pub mod store;

use std::sync::Arc;
use std::cell::RefCell;
pub use store::Registry;

thread_local! {
    static CURRENT_REGISTRY: RefCell<Arc<Registry>> = RefCell::new(Arc::new(Registry::new()));
}

/// Provides a value to the current thread-local context scope.
///
/// # Examples
///
/// ```
/// use rupa_context::{provide_context, use_context};
/// provide_context(42i32);
/// let val = use_context::<i32>().unwrap();
/// assert_eq!(*val, 42);
/// ```
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

/// Executes a closure within a specific context scope, restoring the previous one after.
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

/// Creates a new scoped registry that inherits from the current context.
pub fn create_scoped_registry() -> Arc<Registry> {
    CURRENT_REGISTRY.with(|reg| {
        Arc::new(Registry::with_parent(reg.borrow().clone()))
    })
}

/// A mock context helper for TDD and headless testing.
pub struct MockContext;

impl MockContext {
    /// Runs a closure with a fresh, empty registry.
    pub fn run<F, R>(f: F) -> R 
    where F: FnOnce() -> R {
        with_registry(Arc::new(Registry::new()), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_scoping() {
        MockContext::run(|| {
            provide_context("root".to_string());
            
            let scoped = create_scoped_registry();
            with_registry(scoped, || {
                provide_context("child".to_string());
                assert_eq!(*use_context::<String>().unwrap(), "child");
            });
            
            assert_eq!(*use_context::<String>().unwrap(), "root");
        });
    }
}
