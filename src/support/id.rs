use std::sync::atomic::{AtomicU64, Ordering};

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generates a unique, stable ID for a component.
/// Format: "rupa-{counter}"
pub fn generate_id() -> String {
    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("rupa-{}", id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_uniqueness() {
        let id1 = generate_id();
        let id2 = generate_id();
        assert_ne!(id1, id2);
        assert!(id1.starts_with("rupa-"));
    }
}
