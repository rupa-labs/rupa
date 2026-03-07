use std::collections::HashMap;

/// A container for dynamic component attributes (e.g., data-* attributes).
/// Separated from Style to maintain strict Separation of Concerns (SOC).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Attributes {
    pub map: HashMap<String, String>,
}

impl Attributes {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets an attribute value.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.map.insert(key.into(), value.into());
    }

    /// Gets an attribute value.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_attributes_set_get() {
        let mut attrs = Attributes::new();
        attrs.set("data-id", "123");
        assert_eq!(attrs.get("data-id").unwrap(), "123");
        assert_eq!(attrs.get("non-existent"), None);
    }
}
