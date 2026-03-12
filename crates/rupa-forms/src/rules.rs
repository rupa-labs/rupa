use serde_json::Value;
use std::fmt::Debug;

/// The core interface for validation rules, inspired by Laravel.
pub trait Rule: Send + Sync + Debug {
    /// Determine if the validation rule passes.
    fn passes(&self, value: &Value) -> bool;

    /// Get the validation error message.
    fn message(&self) -> String;
}

// --- Standard Rules ---

#[derive(Debug)]
pub struct Required;
impl Rule for Required {
    fn passes(&self, value: &Value) -> bool {
        match value {
            Value::Null => false,
            Value::String(s) => !s.trim().is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(m) => !m.is_empty(),
            _ => true,
        }
    }
    fn message(&self) -> String { "This field is required.".to_string() }
}

#[derive(Debug)]
pub struct Email;
impl Rule for Email {
    fn passes(&self, value: &Value) -> bool {
        if let Some(s) = value.as_str() {
            // Simple email heuristic for Tier 1
            return s.contains('@') && s.contains('.');
        }
        false
    }
    fn message(&self) -> String { "The field must be a valid email address.".to_string() }
}

#[derive(Debug)]
pub struct Min(pub f64);
impl Rule for Min {
    fn passes(&self, value: &Value) -> bool {
        match value {
            Value::Number(n) => n.as_f64().unwrap_or(0.0) >= self.0,
            Value::String(s) => (s.len() as f64) >= self.0,
            Value::Array(a) => (a.len() as f64) >= self.0,
            _ => false,
        }
    }
    fn message(&self) -> String { format!("The field must be at least {}.", self.0) }
}

#[derive(Debug)]
pub struct Max(pub f64);
impl Rule for Max {
    fn passes(&self, value: &Value) -> bool {
        match value {
            Value::Number(n) => n.as_f64().unwrap_or(0.0) <= self.0,
            Value::String(s) => (s.len() as f64) <= self.0,
            Value::Array(a) => (a.len() as f64) <= self.0,
            _ => true,
        }
    }
    fn message(&self) -> String { format!("The field may not be greater than {}.", self.0) }
}

#[derive(Debug)]
pub struct Regex(pub String, pub String); // Pattern, Custom Message
impl Rule for Regex {
    fn passes(&self, value: &Value) -> bool {
        if let Some(s) = value.as_str() {
            // Note: In production we would use the regex crate, 
            // for now we check for substring as a placeholder.
            return s.contains(&self.0);
        }
        false
    }
    fn message(&self) -> String { self.1.clone() }
}

#[derive(Debug)]
pub struct Numeric;
impl Rule for Numeric {
    fn passes(&self, value: &Value) -> bool {
        value.is_number() || (value.is_string() && value.as_str().unwrap().parse::<f64>().is_ok())
    }
    fn message(&self) -> String { "The field must be a number.".to_string() }
}

#[derive(Debug)]
pub struct Integer;
impl Rule for Integer {
    fn passes(&self, value: &Value) -> bool {
        value.is_i64() || (value.is_string() && value.as_str().unwrap().parse::<i64>().is_ok())
    }
    fn message(&self) -> String { "The field must be an integer.".to_string() }
}

#[derive(Debug)]
pub struct Boolean;
impl Rule for Boolean {
    fn passes(&self, value: &Value) -> bool {
        value.is_boolean() || (value.is_string() && (value.as_str().unwrap() == "true" || value.as_str().unwrap() == "false"))
    }
    fn message(&self) -> String { "The field must be true or false.".to_string() }
}
