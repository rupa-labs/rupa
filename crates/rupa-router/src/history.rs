/// Agnostic interface for managing navigation history.
pub trait History: Send + Sync {
    fn push(&self, path: &str);
    fn replace(&self, path: &str);
    fn back(&self);
}
