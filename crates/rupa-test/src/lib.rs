pub mod headless;
pub mod snapshot;
pub mod interaction;

pub use headless::Tester;
pub use snapshot::Snapshot;

/// A helper to quickly setup a test environment.
pub fn setup() -> Tester {
    Tester::new()
}
