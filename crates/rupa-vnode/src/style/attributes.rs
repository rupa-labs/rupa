use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Attributes {
    pub map: HashMap<String, String>,
}

impl Attributes {
    pub fn new() -> Self { Self::default() }
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.map.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.map.get(key)
    }
}
