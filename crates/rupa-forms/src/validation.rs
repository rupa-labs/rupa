/// Logic for data verification.
pub trait Validator<T>: Send + Sync {
    fn validate(&self, value: &T) -> Result<(), String>;
}
