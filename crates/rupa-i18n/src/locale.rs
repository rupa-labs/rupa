use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Locale {
    pub language: String,
    pub region: String,
}

impl Locale {
    pub fn new(language: impl Into<String>, region: impl Into<String>) -> Self {
        Self {
            language: language.into(),
            region: region.into(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}-{}", self.language, self.region)
    }
}

impl Default for Locale {
    fn default() -> Self {
        Self::new("en", "US")
    }
}
