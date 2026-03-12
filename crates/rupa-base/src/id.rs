use std::sync::atomic::{AtomicU64, Ordering};
use std::fmt;
use serde::{Serialize, Deserialize};

static GLOBAL_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// A lightweight, unique identifier for Rupa components and resources.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Id(u64);

impl Id {
    /// Generates a new unique identifier.
    pub fn next() -> Self {
        Self(GLOBAL_ID_COUNTER.fetch_add(1, Ordering::SeqCst))
    }

    /// Creates an Id from a raw u64. Use with caution.
    pub const fn from_raw(id: u64) -> Self {
        Self(id)
    }

    /// Returns the raw u64 value.
    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::next()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rupa-{}", self.0)
    }
}

/// Helper function to generate a unique ID string.
pub fn generate_id_string() -> String {
    Id::next().to_string()
}
