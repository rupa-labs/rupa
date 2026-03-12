//! # Rupa Forms 📝
//!
//! Reactive Form Management and Validation for the Rupa Framework. 
//! Provides a highly decoupled, schema-driven approach to interactive 
//! data entry and state synchronization.

pub mod validation;
pub mod form_state;
pub mod schema;
pub mod rules;
pub mod form;

pub use form_state::{Manager, Field, Validatable};
pub use validation::Validator;
pub use rules::Rule;
pub use form::Form;
pub use rupa_macros::Form;

use std::sync::{Arc, RwLock};

/// Re-export standard validation rules for convenience.
pub mod std_rules {
    pub use crate::rules::{
        Required, Email, Min, Max, Regex, Numeric, Integer, Boolean
    };
}

/// A reactive scope for form state management.
///
/// Wraps a `Manager` to provide a localized context for form fields 
/// and their associated validation rules.
pub struct Scope {
    /// The underlying reactive form manager.
    pub manager: Manager,
}

impl Scope {
    /// Creates a new form scope with a fresh manager.
    pub fn new() -> Self {
        Self {
            manager: Manager::new(),
        }
    }
}

/// A mock form implementation for TDD and headless testing.
pub struct MockForm {
    pub submitted_data: Arc<RwLock<Option<serde_json::Value>>>,
}

impl MockForm {
    /// Creates a new, empty mock form.
    pub fn new() -> Self {
        Self {
            submitted_data: Arc::new(RwLock::new(None)),
        }
    }

    /// Simulates a form submission.
    pub fn submit(&self, data: serde_json::Value) {
        *self.submitted_data.write().unwrap() = Some(data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_form_flow() {
        let form = MockForm::new();
        let data = serde_json::json!({"email": "artisan@rupa.rs"});
        form.submit(data.clone());
        
        let submitted = form.submitted_data.read().unwrap();
        assert_eq!(submitted.as_ref().unwrap(), &data);
    }
}
