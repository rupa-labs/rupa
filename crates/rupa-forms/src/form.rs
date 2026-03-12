use std::collections::HashMap;
use std::sync::Arc;
use crate::form_state::Validatable;

/// A collection of fields with unified validation logic.
pub trait Form: Send + Sync {
    /// Returns all validatable items in this form.
    fn fields(&self) -> HashMap<String, Arc<dyn Validatable>>;

    /// Validates all fields and triggers error displays.
    fn validate(&self) -> bool {
        let mut valid = true;
        for field in self.fields().values() {
            field.touch();
            if !field.is_valid() {
                valid = false;
            }
        }
        valid
    }

    fn is_dirty(&self) -> bool {
        self.fields().values().any(|f| f.is_dirty())
    }

    fn reset(&self) {
        for field in self.fields().values() {
            field.reset();
        }
    }

    /// Returns a map of current validation errors.
    fn errors(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (name, field) in self.fields() {
            if let Some(err) = field.error() {
                map.insert(name, err);
            }
        }
        map
    }
}
