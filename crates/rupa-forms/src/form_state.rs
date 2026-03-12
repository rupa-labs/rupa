use std::sync::Arc;
use rupa_signals::{Signal, Effect};
use crate::validation::Validator;
use serde::Serialize;

/// A type-erased interface for any validatable item.
pub trait Validatable: Send + Sync {
    fn is_valid(&self) -> bool;
    fn error(&self) -> Option<String>;
    fn is_dirty(&self) -> bool;
    fn is_touched(&self) -> bool;
    fn touch(&self);
    fn reset(&self);
}

/// Reactive state for a single input field.
#[derive(Clone)]
pub struct Field<T> {
    pub value: Signal<T>,
    pub initial: T,
    pub error: Signal<Option<String>>,
    pub is_touched: Signal<bool>,
    pub is_dirty: Signal<bool>,
    pub validator: Arc<Validator>,
}

impl<T: Clone + Send + Sync + Default + Serialize + 'static> Field<T> {
    pub fn new(initial: T, validator: Validator) -> Self {
        let value = Signal::new(initial.clone());
        let initial_val = initial;
        let error = Signal::new(None);
        let is_touched = Signal::new(false);
        let is_dirty = Signal::new(false);
        let validator = Arc::new(validator);

        let v_clone = value.clone();
        let e_clone = error.clone();
        let d_clone = is_dirty.clone();
        let val_clone = validator.clone();

        let _effect = Effect::new(move || {
            let val = v_clone.get();
            let dirty = d_clone.get();
            
            if let Ok(json) = serde_json::to_value(&val) {
                match val_clone.validate(&json, dirty) {
                    Ok(_) => e_clone.set(None),
                    Err(msg) => e_clone.set(Some(msg)),
                }
            }
        });

        Self {
            value,
            initial: initial_val,
            error,
            is_touched,
            is_dirty,
            validator,
        }
    }

    pub fn set(&self, val: T) {
        self.is_dirty.set(true);
        self.value.set(val);
    }

    pub fn touch(&self) {
        self.is_touched.set(true);
    }

    pub fn is_valid(&self) -> bool {
        self.error.get().is_none()
    }
}

impl<T: Clone + Send + Sync + Default + Serialize + 'static> Validatable for Field<T> {
    fn is_valid(&self) -> bool { self.is_valid() }
    fn error(&self) -> Option<String> { self.error.get() }
    fn is_dirty(&self) -> bool { self.is_dirty.get() }
    fn is_touched(&self) -> bool { self.is_touched.get() }
    fn touch(&self) { self.touch() }
    fn reset(&self) {
        self.value.set(self.initial.clone());
        self.is_dirty.set(false);
        self.is_touched.set(false);
        self.error.set(None);
    }
}

/// Orchestrator for form lifecycle.
pub struct Manager {
    pub is_submitting: Signal<bool>,
}

impl Manager {
    pub fn new() -> Self {
        Self {
            is_submitting: Signal::new(false),
        }
    }
}
