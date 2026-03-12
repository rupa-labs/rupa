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

/// Re-export standard rules for convenience.
pub mod std_rules {
    pub use crate::rules::{
        Required, Email, Min, Max, Regex, Numeric, Integer, Boolean
    };
}

/// A scope for reactive form management.
pub struct Scope {
    pub manager: Manager,
}

impl Scope {
    pub fn new() -> Self {
        Self {
            manager: Manager::new(),
        }
    }
}
