use std::sync::Arc;
use serde_json::Value;
use crate::rules::Rule;

/// An orchestrator for running a chain of validation rules.
#[derive(Debug, Clone, Default)]
pub struct Validator {
    pub rules: Vec<Arc<dyn Rule>>,
    pub is_nullable: bool,
    pub is_sometimes: bool,
}

impl Validator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a rule to the chain.
    pub fn rule(mut self, rule: impl Rule + 'static) -> Self {
        self.rules.push(Arc::new(rule));
        self
    }

    /// Allow the field to be null.
    pub fn nullable(mut self) -> Self {
        self.is_nullable = true;
        self
    }

    /// Only validate if the field is present/dirty.
    pub fn sometimes(mut self) -> Self {
        self.is_sometimes = true;
        self
    }

    /// Validate a value against the chain of rules.
    pub fn validate(&self, value: &Value, is_dirty: bool) -> Result<(), String> {
        // Handle 'sometimes'
        if self.is_sometimes && !is_dirty {
            return Ok(());
        }

        // Handle 'nullable'
        if self.is_nullable && value.is_null() {
            return Ok(());
        }

        for rule in &self.rules {
            if !rule.passes(value) {
                return Err(rule.message());
            }
        }

        Ok(())
    }
}
