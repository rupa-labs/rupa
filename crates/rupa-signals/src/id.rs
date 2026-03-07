use std::sync::atomic::{AtomicUsize, Ordering};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generates a unique identifier for any Rupa component or resource.
pub fn generate_id() -> String {
    let id = GLOBAL_ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    format!("rupa-{}", id)
}
