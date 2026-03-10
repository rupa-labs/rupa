use serde::{Serialize, Deserialize};

/// Configuration manifest for a Rupa application (rupa.toml).
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub name: String,
    pub version: String,
}
